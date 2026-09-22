const $ = (id) => document.getElementById(id);
const W = 156;
const H = 42;
const XG = 18;
const YG = 46;
const state = {
  tree: [],
  flat: new Map(),
  expanded: new Set(["__root__"]),
  path: [],
  current: null,
  selected: null,
  view: "tree",
  mode: "search",
  scale: 1,
  running: false,
  abort: null,
  queue: [],
  playing: false,
  progress: [],
  ranked: 0,
  tableQuery: "",
  tablePath: [],
  history: [],
  draft: null,
  root: null,
  rootPath: [],
};

const esc = (v) =>
  String(v ?? "").replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));
const pct = (v) => (v == null || Number.isNaN(Number(v)) ? "" : `${Math.round(Number(v) * 100)}%`);

function flatten(nodes, map = new Map(), parent = null, depth = 0) {
  for (const n of nodes) {
    n.parent_id = n.parent_id ?? parent;
    n.depth = depth;
    map.set(String(n.id), n);
    flatten(n.children || [], map, n.id, depth + 1);
  }
  return map;
}
function descendants(n) {
  return (n.children || []).reduce((a, c) => a + 1 + descendants(c), 0);
}
function currentRootPath() {
  const parts = location.pathname.split("/").filter(Boolean);
  if (!parts.length || ["docs", "api", "static"].includes(parts[0])) return "";
  return parts.join("/");
}
function rootQuery(extra) {
  const params = new URLSearchParams(extra || "");
  const root = currentRootPath();
  if (root) params.set("root", root);
  const s = params.toString();
  return s ? `?${s}` : "";
}
function rootHref(nodeId) {
  if (!nodeId || nodeId === "__root__") return "/";
  if (state.root && nodeId === state.root.id) {
    return currentRootPath() ? `/${currentRootPath()}` : "/";
  }
  const prefix = currentRootPath() ? currentRootPath().split("/") : [];
  const extra = [];
  let n = state.flat.get(String(nodeId));
  const stop = state.root?.id;
  while (n && n.id !== stop && n.id !== "__root__") {
    extra.unshift(n.id);
    n = n.parent_id ? state.flat.get(String(n.parent_id)) : null;
  }
  const segs = prefix.concat(extra);
  return segs.length ? `/${segs.join("/")}` : "/";
}
function visualRootId() {
  return state.root?.id || "__root__";
}
function rootLabel() {
  return state.root?.name || "Knowledge";
}
function rootNode() {
  if (state.root) {
    return {
      ...state.root,
      children: state.tree,
      item_count: state.root.item_count || 0,
      depth: -1,
    };
  }
  return { id: "__root__", name: "Knowledge", children: state.tree, item_count: 0, depth: -1 };
}
const TOKEN_KEY = "jev_tree_token";
let authBusy = false;
const authedFetch = (path, opts = {}) => {
  const token = localStorage.getItem(TOKEN_KEY);
  const headers = { ...(opts.headers || {}) };
  if (token) headers.Authorization = `Bearer ${token}`;
  return fetch(path, { ...opts, headers });
};
async function api(path, opts = {}) {
  const r = await authedFetch(path, opts);
  const d = await r.json().catch(() => ({}));
  if (r.status === 401) throw Error("__login_required__");
  if (!r.ok) throw Error(d.detail || d.message || r.statusText);
  return d;
}
async function requireLogin(site) {
  if (authBusy) return;
  authBusy = true;
  localStorage.removeItem(TOKEN_KEY);
  try {
    await askLogin(site);
  } finally {
    authBusy = false;
  }
}
async function openApi(path, opts = {}) {
  const r = await fetch(path, opts);
  const d = await r.json().catch(() => ({}));
  if (!r.ok) throw Error(d.detail || d.message || r.statusText);
  return d;
}

function visibleKids(n) {
  if (!state.expanded.has(String(n.id))) return [];
  return n.children || [];
}
function measure(n) {
  const kids = visibleKids(n);
  if (!kids.length) return W;
  return Math.max(W, kids.reduce((a, c) => a + measure(c), 0) + XG * (kids.length - 1));
}
function layout(n, x0, depth) {
  const kids = visibleKids(n);
  const nodes = [];
  const edges = [];
  let childLayouts = [];
  let cursor = x0;
  for (const child of kids) {
    const w = measure(child);
    const laid = layout(child, cursor, depth + 1);
    childLayouts.push(laid.node);
    nodes.push(...laid.nodes);
    edges.push(...laid.edges);
    cursor += w + XG;
  }
  const x = kids.length ? (childLayouts[0].x + childLayouts.at(-1).x) / 2 : x0;
  const node = {
    id: String(n.id),
    name: n.name || n.id,
    count: n.item_count || 0,
    kids: (n.children || []).length,
    more: descendants(n),
    x,
    y: depth * (H + YG),
  };
  for (const child of childLayouts) {
    edges.push({
      from: node.id,
      to: child.id,
      x1: node.x + W / 2,
      y1: node.y + H,
      x2: child.x + W / 2,
      y2: child.y,
    });
  }
  return { node, nodes: [node, ...nodes], edges, width: Math.max(measure(n), cursor - x0), height: node.y + H };
}

function short(name, max = 16) {
  const s = String(name || "");
  return s.length > max ? `${s.slice(0, max - 1)}…` : s;
}
function nodeClass(id) {
  if (id === "__root__" || id === visualRootId()) return "root";
  if (state.current && id === state.current) return "current";
  if (state.path.includes(id)) return "path";
  if (state.path.length && !state.path.includes(id) && id !== visualRootId()) return "dim";
  return "";
}
function focusResult(categoryId, itemId) {
  if (categoryId && categoryId !== "__root__" && state.flat.get(String(categoryId))) {
    const path = expandTo(categoryId);
    state.path = path;
    state.current = String(categoryId);
    state.selected = String(categoryId);
    setView("tree");
    render();
    $("stage").textContent = path.map((nid) => state.flat.get(nid)?.name).filter(Boolean).join(" → ");
    requestAnimationFrame(() => {
      const node = $("tree").querySelector(`.node[data-id="${CSS.escape(String(categoryId))}"]`);
      node?.scrollIntoView({ block: "center", inline: "center", behavior: "smooth" });
    });
  }
  $("panelBody")?.querySelectorAll(".item").forEach((el) => {
    el.classList.toggle("active", itemId != null && el.dataset.item === String(itemId));
  });
}
function renderTree() {
  if (state.view !== "tree") return;
  const root = rootNode();
  if (!state.tree.length && !state.root) {
    $("empty").textContent = "No categories";
    $("empty").classList.remove("hidden");
    $("tree").innerHTML = "";
    return;
  }
  $("empty").classList.add("hidden");
  const data = layout(root, 0, 0);
  const width = Math.max(data.width + 80, 640);
  const height = Math.max(...data.nodes.map((n) => n.y)) + H + 80;
  const pathSet = new Set(state.path);
  const rootId = visualRootId();
  const edges = data.edges
    .map((e) => {
      const on = ((e.from === "__root__" || e.from === rootId) && pathSet.has(e.to)) || (pathSet.has(e.from) && pathSet.has(e.to));
      const mid = (e.y1 + e.y2) / 2;
      return `<path class="edge ${on ? "path" : ""}" d="M${e.x1} ${e.y1}V${mid}H${e.x2}V${e.y2}"/>`;
    })
    .join("");
  const nodes = data.nodes
    .map((n) => {
      const cls = nodeClass(n.id);
      const extra = n.kids ? `${n.kids} branches` : n.count ? `${n.count} items` : "";
      return `<g class="node ${cls}" data-id="${esc(n.id)}" transform="translate(${n.x},${n.y})">
        <title>${esc(n.name)}</title>
        <rect width="${W}" height="${H}" rx="8"/>
        <foreignObject x="8" y="6" width="${W - 16}" height="${H - 10}">
          <div xmlns="http://www.w3.org/1999/xhtml" class="label">${esc(n.name)}</div>
          <div xmlns="http://www.w3.org/1999/xhtml" class="meta">${esc(extra)}</div>
        </foreignObject>
      </g>`;
    })
    .join("");
  const pad = 24;
  $("tree").innerHTML = `<svg class="tree" width="${width}" height="${height}" viewBox="${-pad} ${-pad} ${width} ${height}">${edges}${nodes}</svg>`;
  $("tree").querySelectorAll(".node").forEach((el) => {
    el.onclick = () => onNodeClick(el.dataset.id);
  });
}

function tableStats(n) {
  if (n._tableStats) return n._tableStats;
  let items = n.item_count || 0;
  let nodes = 1;
  for (const child of n.children || []) {
    const s = tableStats(child);
    items += s.items;
    nodes += s.nodes;
  }
  n._tableStats = { items, nodes };
  return n._tableStats;
}
function tableHay(n) {
  return `${n.name} ${n.description || ""} ${categoryPath(n.id)}`.toLowerCase();
}
function tableMark(text, q) {
  const s = String(text || "");
  if (!q) return esc(s);
  const i = s.toLowerCase().indexOf(q.toLowerCase());
  if (i < 0) return esc(s);
  return `${esc(s.slice(0, i))}<mark>${esc(s.slice(i, i + q.length))}</mark>${esc(s.slice(i + q.length))}`;
}
function tableMeta(n) {
  const kids = (n.children || []).length;
  const items = n.item_count || 0;
  const stats = tableStats(n);
  if (kids) return `${kids} branches · ${stats.items} items`;
  return items ? `${items} items` : "leaf";
}
function collectTableHits(nodes, q, out = []) {
  for (const n of nodes || []) {
    if (tableHay(n).includes(q)) out.push(n);
    collectTableHits(n.children || [], q, out);
  }
  return out;
}
function tableColumns() {
  const cols = [{ title: rootLabel(), nodes: state.tree, active: state.tablePath[0] || null }];
  for (let i = 0; i < state.tablePath.length; i++) {
    const n = state.flat.get(String(state.tablePath[i]));
    if (!n) break;
    const kids = n.children || [];
    if (!kids.length) break;
    cols.push({ title: n.name, nodes: kids, active: state.tablePath[i + 1] || null });
  }
  return cols;
}
function renderTableItem(n, q, active) {
  const kids = (n.children || []).length;
  const on = active === String(n.id) || state.selected === String(n.id) ? "selected" : "";
  return `<button class="col-item ${on}" type="button" data-id="${esc(n.id)}" title="${esc(n.description || n.name)}">
    <span class="col-name">${tableMark(n.name, q)}</span>
    <span class="col-meta">${esc(tableMeta(n))}</span>
    ${kids ? `<span class="col-more" aria-hidden="true">›</span>` : ""}
  </button>`;
}
function renderTable() {
  if (state.view !== "table") return;
  if ($("tableQuery") && document.activeElement !== $("tableQuery")) $("tableQuery").value = state.tableQuery;
  if (!state.tree.length && !state.root) {
    $("empty").textContent = "No categories";
    $("empty").classList.remove("hidden");
    $("tableCount").textContent = "";
    $("tableTrail").innerHTML = "";
    $("tableScroll").innerHTML = "";
    return;
  }
  $("empty").classList.add("hidden");
  const q = state.tableQuery.trim();
  const total = state.tree.reduce((a, n) => a + tableStats(n).nodes, 0);
  if (q) {
    const hits = collectTableHits(state.tree, q.toLowerCase());
    $("tableCount").textContent = `${hits.length} matches`;
    $("tableTrail").innerHTML = "";
    if (!hits.length) {
      $("tableScroll").innerHTML = `<p class="table-empty">No matching categories</p>`;
      return;
    }
    const body = hits
      .map((n) => {
        const on = state.selected === String(n.id) ? "selected" : "";
        return `<tr class="${on}" data-id="${esc(n.id)}">
          <td class="hit-name">${tableMark(n.name, q)}</td>
          <td class="hit-path">${esc(categoryPath(n.parent_id) || rootLabel())}</td>
          <td class="num">${n.item_count || 0}</td>
          <td class="num">${(n.children || []).length}</td>
        </tr>`;
      })
      .join("");
    $("tableScroll").innerHTML = `<table class="hits"><thead><tr><th>Category</th><th>Path</th><th class="num">Items</th><th class="num">Branches</th></tr></thead><tbody>${body}</tbody></table>`;
    $("tableScroll").querySelectorAll("tr[data-id]").forEach((row) => {
      row.onclick = () => onTableSelect(row.dataset.id);
    });
    return;
  }
  const cols = tableColumns();
  const ancestors = (state.rootPath || []).slice(0, -1);
  const crumb = (state.root ? [`<button type="button" data-root="0">Knowledge</button>`] : [])
    .concat(ancestors.map((n, i) => `<button type="button" data-root="${i + 1}">${esc(n.name || n.id)}</button>`))
    .concat([`<button type="button" data-depth="-1">${esc(rootLabel())}</button>`])
    .concat(
      state.tablePath.map((id, i) => {
        const n = state.flat.get(String(id));
        return `<button type="button" data-depth="${i}">${esc(n?.name || id)}</button>`;
      })
    );
  $("tableTrail").innerHTML = crumb.join(`<span class="sep">/</span>`);
  $("tableCount").textContent = `${total} categories`;
  $("tableScroll").innerHTML = `<div class="cols">${cols
    .map(
      (col) => `<section class="col">
      <h3>${esc(col.title)}</h3>
      <div class="col-list">${col.nodes.map((n) => renderTableItem(n, "", col.active)).join("")}</div>
    </section>`
    )
    .join("")}</div>`;
  $("tableScroll").querySelectorAll(".col-item").forEach((el) => {
    el.onclick = () => onTableSelect(el.dataset.id);
  });
  $("tableTrail").querySelectorAll("button[data-depth]").forEach((btn) => {
    btn.onclick = () => {
      const depth = Number(btn.dataset.depth);
      state.tablePath = depth < 0 ? [] : state.tablePath.slice(0, depth + 1);
      const last = state.tablePath.at(-1);
      state.selected = last || null;
      renderTable();
      if (last) {
        const node = state.flat.get(String(last));
        if (node) showNode(node);
      }
    };
  });
  $("tableTrail").querySelectorAll("button[data-root]").forEach((btn) => {
    btn.onclick = () => {
      const n = Number(btn.dataset.root);
      const segs = (state.rootPath || []).slice(0, n).map((p) => p.id);
      history.pushState({}, "", segs.length ? `/${segs.join("/")}` : "/");
      loadTree();
    };
  });
  requestAnimationFrame(() => {
    const box = $("tableScroll");
    box.scrollLeft = box.scrollWidth;
    box.querySelector(".col-item.selected")?.scrollIntoView({ block: "nearest", inline: "nearest" });
  });
}
async function onTableSelect(id) {
  const node = state.flat.get(String(id));
  if (!node) return;
  const path = [];
  let n = node;
  const stop = visualRootId();
  while (n && String(n.id) !== stop) {
    path.unshift(String(n.id));
    n = n.parent_id ? state.flat.get(String(n.parent_id)) : null;
  }
  state.tablePath = path;
  state.selected = String(id);
  if (state.tableQuery) {
    state.tableQuery = "";
    if ($("tableQuery")) $("tableQuery").value = "";
  }
  renderTable();
  await showNode(node);
}
function render() {
  if (state.view === "table") renderTable();
  else renderTree();
}
function setView(view) {
  state.view = view;
  $("treeView").classList.toggle("active", view === "tree");
  $("tableView").classList.toggle("active", view === "table");
  $("tree").classList.toggle("hidden", view !== "tree");
  $("table").classList.toggle("hidden", view === "tree");
  render();
}

function expandTo(id) {
  let n = state.flat.get(String(id));
  const ids = [];
  const stop = state.root?.id;
  while (n && n.id !== stop) {
    ids.unshift(String(n.id));
    state.expanded.add(String(n.id));
    n = n.parent_id ? state.flat.get(String(n.parent_id)) : null;
  }
  state.expanded.add(visualRootId());
  state.expanded.add("__root__");
  return ids;
}
function applyRootHref(href) {
  if (href === location.pathname) return false;
  history.pushState({}, "", href);
  loadTree().catch((e) => {
    $("empty").textContent = e.message;
    $("empty").classList.remove("hidden");
  });
  return true;
}
function enterRoot(id) {
  return applyRootHref(rootHref(id));
}
function leaveRoot() {
  const parts = currentRootPath().split("/").filter(Boolean);
  if (!parts.length) return false;
  parts.pop();
  return applyRootHref(parts.length ? `/${parts.join("/")}` : "/");
}
async function onNodeClick(id) {
  if (id === "__root__" || id === visualRootId()) {
    if (state.root) {
      leaveRoot();
      return;
    }
    state.selected = null;
    setPanelOpen(false);
    render();
    return;
  }
  const node = state.flat.get(String(id));
  if (!node) return;
  state.selected = String(id);
  if ((node.children || []).length) {
    if (state.expanded.has(String(id))) state.expanded.delete(String(id));
    else state.expanded.add(String(id));
  }
  render();
  await showNode(node);
}
async function showNode(node) {
  setPanelOpen(true);
  $("panelTitle").textContent = node.name;
  $("panelMeta").textContent = `${(node.children || []).length} children · ${node.item_count || 0} items`;
  $("panelBody").innerHTML = "Loading…";
  const canEnter = node.id !== visualRootId() && node.id !== "__root__";
  const open = canEnter
    ? `<p class="kicker"><button id="openAsRoot" type="button">Use as root</button></p>`
    : "";
  try {
    const d = await api(`/api/items${rootQuery(`limit=20&scope=exact&category_id=${encodeURIComponent(node.id)}`)}`);
    const items = d.items || [];
    $("panelBody").innerHTML =
      open +
      (items.length
        ? items.map((item) => `<div class="item"><b>${esc(item.question)}</b><p>${esc(item.answer)}</p></div>`).join("")
        : "<p>No items stored in this category yet.</p>");
    if (canEnter) $("openAsRoot").onclick = () => enterRoot(node.id);
  } catch (e) {
    $("panelBody").innerHTML = open + `<p>${esc(e.message)}</p>`;
    if (canEnter) $("openAsRoot").onclick = () => enterRoot(node.id);
  }
}

function categoryPath(id) {
  const names = [];
  let n = state.flat.get(String(id));
  let guard = 0;
  const stop = state.root?.id;
  while (n && n.id !== stop && guard++ < 20) {
    names.unshift(n.name);
    n = n.parent_id ? state.flat.get(String(n.parent_id)) : null;
  }
  if (state.root && names.length) names.unshift(state.root.name);
  return names.join(" → ");
}
function roleLabel(role) {
  return { recommended: "Recommended", alternative: "Alternative", reference: "Reference" }[role] || "";
}
function resultCard(r, extraClass = "") {
  const item = r.item || {};
  const why = [roleLabel(r.role), r.relevance != null && `relevant ${pct(r.relevance)}`]
    .filter(Boolean)
    .join(" · ");
  return `<article class="item ${extraClass}" data-category="${esc(item.category_id || "")}" data-item="${esc(item.id || "")}" role="button" tabindex="0">
    <div class="item-head"><span class="score">${pct(r.score)}</span></div>
    <b>${esc(item.question || "")}</b>
    <p>${esc(item.answer || "")}</p>
    ${why ? `<p class="why">${esc(why)}</p>` : ""}
  </article>`;
}
function showSearchResults(e) {
  const results = [...(e.items || e.results || [])].sort((a, b) => (b.score || 0) - (a.score || 0));
  setPanelOpen(true);
  $("panelTitle").textContent = "Results";
  $("panelMeta").textContent = e.abstained
    ? `Abstained · ${results.length} items`
    : `${results.length} items`;
  if (!results.length) {
    $("panelBody").innerHTML = "<p>No item is ready to send as a reply.</p>";
    return;
  }
  const [best, ...rest] = results;
  const groups = new Map();
  for (const r of rest) {
    const id = String(r.item?.category_id || "");
    if (!groups.has(id)) groups.set(id, []);
    groups.get(id).push(r);
  }
  const grouped = [...groups.entries()]
    .map(([id, items]) => {
      const name = state.flat.get(id)?.name || "Other";
      const path = categoryPath(id);
      const cards = items.map((r) => resultCard(r)).join("");
      return `<section class="group"><h3>${esc(name)}<small>${esc(path)}</small></h3>${cards}</section>`;
    })
    .join("");
  $("panelBody").innerHTML = `<section class="best"><p class="kicker">Best match</p>${resultCard(best, "best-item")}</section>${grouped}`;
  $("panelBody").querySelectorAll(".item[data-item]").forEach((el) => {
    const activate = () => focusResult(el.dataset.category, el.dataset.item);
    el.onclick = activate;
    el.onkeydown = (ev) => {
      if (ev.key === "Enter" || ev.key === " ") {
        ev.preventDefault();
        activate();
      }
    };
  });
  if (best?.item) focusResult(best.item.category_id, best.item.id);
}
function setMode(mode) {
  if (state.running) return;
  state.mode = mode;
  $("searchMode").classList.toggle("active", mode === "search");
  $("ingestMode").classList.toggle("active", mode === "ingest");
  $("searchFields").classList.toggle("hidden", mode !== "search");
  $("ingestFields").classList.toggle("hidden", mode !== "ingest");
  $("run").textContent = mode === "search" ? "Search" : "Ingest";
}

function setProgress(title, meta, line) {
  $("stage").textContent = line || title;
  if (!state.running) return;
  setPanelOpen(true);
  $("panelTitle").textContent = title;
  $("panelMeta").textContent = meta || "";
  if (line) {
    const log = state.progress;
    if (log.at(-1) !== line) log.push(line);
    $("panelBody").innerHTML =
      `<div class="progress">${log.map((step, i) =>
        `<div class="step${i === log.length - 1 ? " now" : ""}">${esc(step)}</div>`
      ).join("")}</div>`;
  }
}
function showEvent(e) {
  if (e.status === "asking") {
    const n = Number(e.depth || 0) + 1;
    setProgress(state.mode === "ingest" ? "Ingesting" : "Searching", `step ${n}`, `Choosing a branch · step ${n}`);
  }
  if (e.status === "answered") {
    const n = (e.nodes || []).at(-1);
    if (n?.choice_id) {
      const path = expandTo(n.choice_id);
      state.path = path;
      state.current = String(n.choice_id);
      const label = `${n.node_name || rootLabel()} → ${n.choice_name || ""}`;
      setProgress(state.mode === "ingest" ? "Ingesting" : "Searching", label, label);
      render();
    } else if (n?.choice_name) {
      setProgress(state.mode === "ingest" ? "Ingesting" : "Searching", n.choice_name, `Stopped here · ${n.choice_name}`);
    }
  }
  if (e.type === "retrieval_completed") {
    const n = Number(e.count || 0);
    setProgress("Searching", n ? `${n} candidates` : "no candidates", n ? `Ranking items · ${n}` : "no items in this category");
  }
  if (e.type === "candidate_evaluated") {
    state.ranked = (state.ranked || 0) + 1;
    $("panelMeta").textContent = `${state.ranked} ranked`;
  }
  if (e.type === "descent_done" && Array.isArray(e.path)) {
    state.path = e.path.map((x) => String(x.id));
    state.current = state.path.at(-1) || null;
    state.path.forEach((id) => state.expanded.add(id));
    const label = state.path
      .map((id) => state.flat.get(id)?.name)
      .filter(Boolean)
      .join(" → ");
    $("stage").textContent = label || "Routed";
    render();
    // Ingest keeps going: `ingest_draft` or `ingest_saved` owns the panel from here.
  }
  if (e.type === "search_done") {
    if (e.query) {
      state.history.push({ role: "user", text: e.query });
      if (state.history.length > 16) state.history.splice(0, state.history.length - 16);
    }
    showSearchResults(e);
  }
  if (e.type === "ingest_draft") {
    showDraft(e);
  }
  if (e.type === "ingest_saved") {
    const item = e.item || {};
    const path = pathLabel(e.path);
    const fromDraft = !!state.draft;
    state.draft = null;
    $("stage").textContent = fromDraft
      ? "Draft published"
      : e.updated
        ? "Existing item updated"
        : "New item stored";
    setPanelOpen(true);
    $("panelTitle").textContent = fromDraft ? "Published" : e.updated ? "Item updated" : "Item added";
    $("panelMeta").textContent = path || "Saved";
    $("panelBody").innerHTML = `<div class="item"><b>${esc(item.question || "")}</b><p>${esc(item.answer || "")}</p></div>`;
    loadTree();
  }
}

function pathLabel(path) {
  return (path || [])
    .map((p) => p.name || state.flat.get(p.id)?.name)
    .filter(Boolean)
    .join(" → ");
}

// Ingest is two steps: the server files a draft, the person approves it. The draft row
// already exists (status=draft, invisible to search) so nothing is lost by walking away.
function showDraft(e) {
  const item = e.item || {};
  state.draft = { id: item.id, version: item.version, question: item.question, answer: item.answer };
  $("stage").textContent = "Review the draft";
  setPanelOpen(true);
  $("panelTitle").textContent = "Draft review";
  $("panelMeta").textContent = pathLabel(e.path) || "Routed";
  const duplicates = e.duplicate_ids || [];
  $("panelBody").innerHTML = `
    <div class="item"><b>${esc(item.question || "")}</b><p>${esc(item.answer || "")}</p></div>
    <p class="kicker">Not visible in search until you publish it.</p>
    ${duplicates.length ? `<p class="kicker">Possible duplicates: ${esc(duplicates.join(", "))}</p>` : ""}
    <div class="draft-actions">
      <button id="draftPublish" type="button">Publish</button>
      <button id="draftDiscard" type="button" class="ghost">Discard</button>
    </div>`;
  $("draftPublish").onclick = publishDraft;
  $("draftDiscard").onclick = discardDraft;
}

async function publishDraft() {
  const draft = state.draft;
  if (!draft || state.running) return;
  setRunning(true);
  try {
    const body = {
      mode: "ingest",
      question: draft.question,
      answer: draft.answer,
      item_id: draft.id,
      expected_version: draft.version,
      auto_publish: true,
    };
    const root = currentRootPath();
    if (root) body.root = root;
    await streamRequest(body);
  } catch (err) {
    $("stage").textContent = err.message;
  } finally {
    setRunning(false);
  }
}

async function discardDraft() {
  const draft = state.draft;
  if (!draft) return;
  try {
    await api(`/api/items/${encodeURIComponent(draft.id)}`, { method: "DELETE" });
    state.draft = null;
    $("stage").textContent = "Draft discarded";
    closePanel();
  } catch (err) {
    $("stage").textContent = err.message;
  }
}

function parseSSE() {
  let buffer = "";
  return {
    push(chunk) {
      buffer += chunk;
      const out = [];
      const parts = buffer.split(/\r?\n\r?\n/);
      buffer = parts.pop() || "";
      for (const block of parts) {
        const data = block
          .split(/\r?\n/)
          .filter((line) => line.startsWith("data:"))
          .map((line) => line.slice(5).trim())
          .join("\n");
        if (!data) continue;
        try {
          out.push(JSON.parse(data));
        } catch {
          out.push({ type: "error", message: "malformed response" });
        }
      }
      return out;
    },
    end() {
      if (!buffer.trim()) return [];
      const data = buffer
        .split(/\r?\n/)
        .filter((line) => line.startsWith("data:"))
        .map((line) => line.slice(5).trim())
        .join("\n");
      if (!data) return [];
      try {
        return [JSON.parse(data)];
      } catch {
        return [{ type: "error", message: "truncated response" }];
      }
    },
  };
}
async function streamRequest(body) {
  const response = await authedFetch("/api/run/stream", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
    signal: state.abort?.signal,
  });
  if (response.status === 401) {
    await requireLogin(auth.status?.site);
    throw Error("login required");
  }
  if (!response.ok || !response.body) {
    let detail = `server error ${response.status}`;
    try {
      const data = await response.json();
      if (data && data.detail) detail = data.detail;
    } catch { /* status line is enough */ }
    throw Error(detail);
  }
  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  const parser = parseSSE();
  let terminal = null;
  for (;;) {
    const { done, value } = await reader.read();
    const chunk = done ? decoder.decode() : decoder.decode(value, { stream: true });
    for (const e of done ? parser.end() : parser.push(chunk)) {
      if (e.type === "error") throw Error(e.message || "run failed");
      showEvent(e);
      if (["search_done", "ingest_saved", "ingest_draft"].includes(e.type)) terminal = e.type;
    }
    if (done) break;
  }
  if (!terminal) throw Error("run ended before a terminal event");
}
function setRunning(on) {
  state.running = on;
  document.body.classList.toggle("busy", on);
  ["query", "question", "answer"].forEach((id) => {
    const el = $(id);
    if (el) el.disabled = on;
  });
  $("searchMode").disabled = on;
  $("ingestMode").disabled = on;
  $("run").classList.toggle("hidden", on);
  $("cancel").classList.toggle("hidden", !on);
}

async function run(ev) {
  ev?.preventDefault();
  if (state.running) return;
  // Ingest always starts as a draft; publishing is a separate, explicit step.
  const body = { mode: state.mode };
  const root = currentRootPath();
  if (root) body.root = root;
  state.draft = null;
  if (state.history.length) body.context = state.history.slice(-8);
  if (state.mode === "ingest") {
    body.question = $("question").value.trim();
    body.answer = $("answer").value.trim();
    if (!body.question || !body.answer) {
      $("stage").textContent = "Question and answer are both required";
      return;
    }
  } else {
    body.query = $("query").value.trim();
    if (!body.query) return;
  }
  state.abort = new AbortController();
  state.queue = [];
  state.progress = [];
  state.ranked = 0;
  state.path = [];
  state.current = null;
  openDefaults();
  render();
  setRunning(true);
  setProgress(state.mode === "ingest" ? "Ingesting" : "Searching", "Ready", "Walking the tree");
  try {
    await streamRequest(body);
  } catch (e) {
    const aborted = e.name === "AbortError";
    $("stage").textContent = aborted ? "Stopped" : e.message;
    setPanelOpen(true);
    $("panelTitle").textContent = aborted ? "Stopped" : "Run failed";
    $("panelMeta").textContent = "";
    $("panelBody").innerHTML = `<p>${esc(aborted ? "Stopped" : e.message)}</p>`;
  } finally {
    state.abort = null;
    setRunning(false);
  }
}

function setPanelOpen(open) {
  $("panel").classList.toggle("hidden", !open);
  $("viewport").classList.toggle("with-panel", !!open);
}
function closePanel() {
  setPanelOpen(false);
  state.selected = null;
}
function openDefaults() {
  state.expanded = new Set([visualRootId(), "__root__"]);
}
function fit() {
  openDefaults();
  state.path = [];
  state.current = null;
  $("stage").textContent = "All categories";
  render();
}
async function loadTree() {
  const d = await api(`/api/tree${rootQuery()}`);
  state.tree = d.tree || [];
  state.root = d.root || null;
  state.rootPath = d.path || [];
  state.flat = flatten(state.tree);
  if (state.root) state.flat.set(String(state.root.id), { ...state.root, children: state.tree });
  state.tablePath = [];
  state.path = [];
  state.current = null;
  openDefaults();
  const trail = (state.rootPath || []).map((n) => n.name).filter(Boolean).join(" → ");
  $("stage").textContent = trail || "Expand a category or search below";
  const brand = document.querySelector(".brand");
  if (brand) brand.setAttribute("href", "/");
  render();
}

function wire() {
  $("searchMode").onclick = () => setMode("search");
  $("ingestMode").onclick = () => setMode("ingest");
  $("bar").onsubmit = run;
  $("cancel").onclick = () => state.abort?.abort();
  $("treeView").onclick = () => setView("tree");
  $("tableView").onclick = () => setView("table");
  $("tableQuery").oninput = () => {
    state.tableQuery = $("tableQuery").value;
    renderTable();
  };
  $("zoomIn").onclick = () => {
    const svg = $("tree").querySelector("svg");
    if (!svg) return;
    state.scale = Math.min(1.8, state.scale + 0.12);
    svg.style.transform = `scale(${state.scale})`;
    svg.style.transformOrigin = "top center";
  };
  $("zoomOut").onclick = () => {
    const svg = $("tree").querySelector("svg");
    if (!svg) return;
    state.scale = Math.max(0.6, state.scale - 0.12);
    svg.style.transform = `scale(${state.scale})`;
    svg.style.transformOrigin = "top center";
  };
  $("closePanel").onclick = () => {
    closePanel();
  };
  window.addEventListener("popstate", () => {
    loadTree().catch((e) => {
      $("empty").textContent = e.message;
      $("empty").classList.remove("hidden");
    });
  });
}

// ---------------------------------------------------------------------------
// Setup / auth / settings
// ---------------------------------------------------------------------------

const auth = {
  status: null, // GET /api/setup/status
  token: () => localStorage.getItem(TOKEN_KEY),
};

function applySite(site) {
  const name = (site?.site_name || "").trim() || "jev-tree";
  const desc = (site?.site_description || "").trim();
  const logo = (site?.site_logo || "").trim();
  document.title = desc ? `${name} — ${desc}` : `${name} — Jev-driven tree descent`;
  const brand = document.querySelector(".brand");
  if (brand) {
    brand.innerHTML = logo
      ? `<img src="${esc(logo)}" width="24" height="24" alt="">${esc(name)}`
      : `<img src="/static/favicon-32.png" width="24" height="24" alt="">${esc(name)}`;
  }
  const meta = document.querySelector('meta[name="description"]');
  if (meta && desc) meta.content = desc;
  for (const sel of ['meta[property="og:title"]', 'meta[name="twitter:title"]']) {
    const el = document.querySelector(sel);
    if (el) el.content = `${name} — Jev-driven tree descent over knowledge`;
  }
  for (const sel of ['meta[property="og:description"]', 'meta[name="twitter:description"]']) {
    const el = document.querySelector(sel);
    if (el && desc) el.content = desc;
  }
  if (!desc) {
    $("stage").textContent = "Expand a category or search below";
  } else if (!$("stage").dataset.touched) {
    $("stage").textContent = desc;
  }
}

function fieldHtml({ id, label, type = "text", placeholder = "", hint = "", value = "" }) {
  return `<div class="field"><label for="${id}">${esc(label)}</label>` +
    `<input id="${id}" type="${type}" placeholder="${esc(placeholder)}" value="${esc(value)}">` +
    (hint ? `<div class="hint">${hint}</div>` : "") + `</div>`;
}

function showObError(msg) {
  const el = $("obError");
  el.textContent = msg || "";
  el.classList.toggle("hidden", !msg);
}
function showSetError(msg) {
  const el = $("setError");
  el.textContent = msg || "";
  el.classList.toggle("hidden", !msg);
}

// --- onboarding wizard: login password → Jev key → optional LLM/brand ---
const onboard = {
  step: 0,
  steps: [],
  generatedKey: "",
  data: { serverKey: "", jevKey: "", baseUrl: "", token: "", model: "", siteName: "", siteDesc: "", siteLogo: "" },
  models: [],
};

function buildOnboardSteps(status) {
  const steps = [];
  if (!status.has_server_key) steps.push("access");
  else if (!auth.token()) steps.push("login");
  if (!status.jev_api_key_set) steps.push("jev");
  steps.push("optional");
  return steps;
}

function renderObSteps() {
  $("obSteps").innerHTML = onboard.steps
    .map((_, i) => `<span class="${i <= onboard.step ? "done" : ""}"></span>`)
    .join("");
  // First step (or solo login gate): no back button at all, Next docks right.
  const hasBack = onboard.steps.length > 1 && onboard.step > 0;
  $("obBack").classList.toggle("hidden", !hasBack);
  $("obNext").textContent = onboard.step === onboard.steps.length - 1 ? "Get started" : "Next";
}

function bindOnboardEnter() {
  $("obBody")?.querySelectorAll("input").forEach((input) => {
    input.onkeydown = (ev) => {
      if (ev.key !== "Enter") return;
      ev.preventDefault();
      $("obNext")?.click();
    };
  });
}

function renderOnboard() {
  renderObSteps();
  showObError("");
  const kind = onboard.steps[onboard.step];
  const body = $("obBody");
  if (kind === "access") {
    $("obTitle").textContent = "Create a password";
    $("obDesc").textContent = "This password opens the app. You will need it on later visits. Set one yourself or generate it.";
    body.innerHTML =
      fieldHtml({ id: "obServerKey", label: "Password (6+ characters)", placeholder: "Leave blank to generate", value: onboard.data.serverKey }) +
      `<div class="row" style="margin-top:0"><button id="obGen" type="button" class="ghost">Generate</button></div>` +
      `<div id="obKeyBox" class="keybox hidden"><code id="obKeyText"></code><button id="obCopy" type="button">Copy</button></div>` +
      `<div class="hint" style="font-size:11px;color:var(--muted);margin-top:8px">The generated password is shown once on this screen. Store it somewhere safe.</div>`;
    $("obGen").onclick = () => {
      // Client-side preview only; the server issues the real key on submit.
      // Same shape: jev_tree_sk_ + 32 random bytes in base64url.
      const bytes = crypto.getRandomValues(new Uint8Array(32));
      let bin = "";
      bytes.forEach((b) => { bin += String.fromCharCode(b); });
      const preview = `jev_tree_sk_${btoa(bin).replaceAll("+", "-").replaceAll("/", "_").replaceAll("=", "")}`;
      $("obServerKey").value = preview;
    };
  } else if (kind === "login") {
    $("obTitle").textContent = "Log in";
    $("obDesc").textContent = "Enter the login password.";
    body.innerHTML = fieldHtml({ id: "obLoginKey", label: "Password", type: "password" });
  } else if (kind === "jev") {
    $("obTitle").textContent = "Connect a Jev key";
    $("obDesc").textContent = "Search and ingest need a TypeSafe Jev key. Ranking always uses Jev. An LLM, configured later, only chooses the category.";
    body.innerHTML = fieldHtml({
      id: "obJevKey", label: "Jev API key", type: "password",
      placeholder: "typesafe key", value: onboard.data.jevKey,
      hint: "Required. The key is encrypted in the database. Search and ingest return an error until it is saved.",
    });
  } else {
    $("obTitle").textContent = "Optional settings";
    $("obDesc").textContent = "You can skip this and change it later in Settings.";
    body.innerHTML =
      fieldHtml({ id: "obBaseUrl", label: "LLM base URL (optional)", placeholder: "https://api.openai.com/v1", value: onboard.data.baseUrl }) +
      fieldHtml({ id: "obToken", label: "LLM token (optional)", type: "password", placeholder: "routes search when a model is set", value: onboard.data.token }) +
      `<div class="field"><label for="obModel">Model (optional)</label>` +
      `<div class="row" style="margin-top:0"><select id="obModel" style="flex:1"></select>` +
      `<button id="obFetchModels" type="button" class="ghost">Fetch</button></div>` +
      `<div class="hint">Loads the LLM model list from the base URL and token. Failures leave the list empty.</div></div>` +
      fieldHtml({ id: "obSiteName", label: "Site name (optional)", placeholder: "jev-tree", value: onboard.data.siteName }) +
      fieldHtml({ id: "obSiteDesc", label: "Description (optional)", placeholder: "One-line intro", value: onboard.data.siteDesc });
    $("obFetchModels").onclick = () => fetchModelsInto($("obModel"), $("obBaseUrl").value.trim(), $("obToken").value.trim());
  }
  bindOnboardEnter();
  const focus = body.querySelector("input, select");
  focus?.focus();
}

function collectOnboard() {
  const kind = onboard.steps[onboard.step];
  if (kind === "access") onboard.data.serverKey = $("obServerKey").value.trim();
  if (kind === "jev") onboard.data.jevKey = $("obJevKey").value.trim();
  if (kind === "optional") {
    onboard.data.baseUrl = $("obBaseUrl").value.trim();
    onboard.data.token = $("obToken").value.trim();
    onboard.data.model = $("obModel").value;
    onboard.data.siteName = $("obSiteName").value.trim();
    onboard.data.siteDesc = $("obSiteDesc").value.trim();
  }
}

function showModelError(msg) {
  ($("settings").classList.contains("hidden") ? showObError : showSetError)(msg);
}

async function fetchModelsInto(select, baseUrl, token, selected) {
  select.innerHTML = "";
  if (!baseUrl) {
    showModelError("Enter an LLM base URL, then fetch.");
    return;
  }
  select.innerHTML = `<option value="">Loading…</option>`;
  try {
    const d = await api("/api/llm/models", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ base_url: baseUrl, token: token || null }),
    });
    const models = d.models || [];
    if (!models.length) {
      select.innerHTML = "";
      showModelError("Model list is empty. Check the URL and token.");
      return;
    }
    select.innerHTML = models.map((m) =>
      `<option value="${esc(m.name)}">${esc(m.name)}${m.description ? ` — ${esc(m.description)}` : ""}</option>`
    ).join("");
    if (selected && ![...select.options].some((o) => o.value === selected)) {
      select.insertAdjacentHTML("afterbegin", `<option value="${esc(selected)}">${esc(selected)}</option>`);
    }
    if (selected) select.value = selected;
  } catch (e) {
    select.innerHTML = "";
    showModelError(e.message === "__login_required__" ? "login required" : `model list failed: ${e.message}`);
  }
}

async function submitOnboard() {
  collectOnboard();
  const kind = onboard.steps[onboard.step];
  try {
    if (kind === "access") {
      const d = await openApi("/api/setup/server-key", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ server_key: onboard.data.serverKey || null }),
      });
      if (d.server_key) {
        onboard.generatedKey = d.server_key;
        $("obKeyText").textContent = d.server_key;
        $("obKeyBox").classList.remove("hidden");
        $("obCopy").onclick = async () => {
          try {
            await navigator.clipboard.writeText(d.server_key);
          } catch {
            // clipboard API unavailable (non-secure context): select for manual copy
            const range = document.createRange();
            range.selectNodeContents($("obKeyText"));
            const sel = getSelection();
            sel?.removeAllRanges();
            sel?.addRange(range);
          }
          const btn = $("obCopy");
          btn.textContent = "Copied ✓";
          btn.classList.add("copied");
        };
      }
      if (d.token) localStorage.setItem(TOKEN_KEY, d.token);
      return true; // stay on step so the user can copy the key, Next advances
    }
    if (kind === "login") {
      const key = $("obLoginKey").value.trim();
      if (!key) { showObError("Enter a password."); return false; }
      const d = await openApi("/api/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ server_key: key }),
      });
      localStorage.setItem(TOKEN_KEY, d.token);
      return true;
    }
    if (kind === "jev") {
      if (!onboard.data.jevKey) {
        showObError("A Jev API key is required.");
        return false;
      }
      await api("/api/settings", {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ jev_api_key: onboard.data.jevKey }),
      });
      return true;
    }
    // optional step
    const patch = {};
    if (onboard.data.baseUrl) patch.llm_base_url = onboard.data.baseUrl;
    if (onboard.data.token) patch.llm_token = onboard.data.token;
    if (onboard.data.model) patch.llm_model = onboard.data.model;
    if (onboard.data.siteName) patch.site_name = onboard.data.siteName;
    if (onboard.data.siteDesc) patch.site_description = onboard.data.siteDesc;
    if (Object.keys(patch).length) {
      await api("/api/settings", {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(patch),
      });
    }
    return true;
  } catch (e) {
    showObError(e.message === "__login_required__" ? "login required" : e.message);
    return false;
  }
}

async function startOnboard(status) {
  onboard.steps = buildOnboardSteps(status);
  onboard.step = 0;
  $("onboard").classList.remove("hidden", "login-only");
  renderOnboard();
  $("obBack").onclick = () => {
    if (onboard.step > 0) { onboard.step -= 1; renderOnboard(); }
  };
  $("obNext").onclick = async () => {
    // access step: first click submits, second click advances (key copy time)
    const kind = onboard.steps[onboard.step];
    if (kind === "access" && !$("obKeyBox").classList.contains("hidden") && $("obNext").dataset.done) {
      onboard.step += 1;
      delete $("obNext").dataset.done;
      renderOnboard();
      return;
    }
    const ok = await submitOnboard();
    if (!ok) return;
    if (kind === "access" && onboard.generatedKey) {
      $("obNext").dataset.done = "1";
      $("obNext").textContent = "Password saved — Next";
      return;
    }
    if (onboard.step < onboard.steps.length - 1) {
      onboard.step += 1;
      renderOnboard();
    } else {
      $("onboard").classList.add("hidden");
      await bootMain();
    }
  };
}

// --- login gate (password exists but no session) ---
async function askLogin(site) {
  onboard.steps = ["login"];
  onboard.step = 0;
  $("obSteps").innerHTML = "";
  $("obTitle").textContent = site?.site_name ? `Log in to ${site.site_name}` : "Log in";
  $("obDesc").textContent = "Enter the login password.";
  $("obBody").innerHTML = fieldHtml({
    id: "obLoginKey",
    label: "Password",
    type: "password",
    placeholder: "Password",
  });
  $("obBack").classList.add("hidden");
  $("obNext").textContent = "Log in";
  $("onboard").classList.add("login-only");
  $("onboard").classList.remove("hidden");
  bindOnboardEnter();
  $("obLoginKey")?.focus();
  $("obNext").onclick = async () => {
    const key = $("obLoginKey").value.trim();
    if (!key) { showObError("Enter a password."); return; }
    try {
      const d = await openApi("/api/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ server_key: key }),
      });
      localStorage.setItem(TOKEN_KEY, d.token);
      $("onboard").classList.add("hidden");
      $("onboard").classList.remove("login-only");
      await bootMain();
    } catch (e) {
      showObError(e.message);
    }
  };
}

// --- settings modal ---
const settingsUI = { tab: "site", data: null };

function renderSettings() {
  const d = settingsUI.data || {};
  document.querySelectorAll("#settings .tabs button").forEach((b) =>
    b.classList.toggle("active", b.dataset.tab === settingsUI.tab));
  const body = $("setBody");
  showSetError("");
  if (settingsUI.tab === "site") {
    body.innerHTML =
      fieldHtml({ id: "setSiteName", label: "Site name", placeholder: "jev-tree", value: d.site_name || "" }) +
      fieldHtml({ id: "setSiteDesc", label: "Description", placeholder: "One-line intro (optional)", value: d.site_description || "" }) +
      `<div class="field"><label for="setLogo">OG / logo image (optional, 500 KB max)</label>` +
      `<input id="setLogo" type="file" accept="image/png,image/jpeg,image/svg+xml,image/webp">` +
      `<div class="hint">Uploads are stored as a data URL. Clear it with the button below.</div>` +
      (d.site_logo ? `<img class="logo-preview" src="${esc(d.site_logo)}" alt="">` : "") +
      (d.site_logo ? `<div class="row" style="margin-top:8px"><button id="setLogoClear" type="button" class="ghost">Clear logo</button></div>` : "") +
      `</div>`;
    const clear = $("setLogoClear");
    if (clear) clear.onclick = () => { settingsUI.clearLogo = true; renderSettings(); };
    if (d.site_logo && !settingsUI.clearLogo) {
      const img = body.querySelector(".logo-preview");
      if (img) img.src = d.site_logo;
    }
    if (settingsUI.clearLogo) {
      const img = body.querySelector(".logo-preview");
      if (img) img.remove();
    }
  } else if (settingsUI.tab === "models") {
    body.innerHTML =
      fieldHtml({
        id: "setJevKey", label: `Jev API key (${d.jev_api_key_set ? `saved ${esc(d.jev_api_key_hint || "")}` : "required"})`,
        type: "password", placeholder: d.jev_api_key_set ? "(enter only to change)" : "typesafe key",
        hint: "Required for search, ingest, and ranking. Clearing it stops runs until a key is saved. Encrypted in the database.",
      }) +
      fieldHtml({ id: "setBaseUrl", label: "LLM base URL", placeholder: "https://api.openai.com/v1", value: d.llm_base_url || "" }) +
      fieldHtml({
        id: "setToken", label: `LLM token${d.llm_token_set ? ` · saved ${esc(d.llm_token_hint || "")}` : ""}`,
        type: "password", placeholder: d.llm_token_set ? "(enter only to change)" : "optional",
        hint: "Optional. With a base URL and a model, this LLM chooses the category. If the token is missing or the call fails, Jev walks the tree instead. Ranking always uses Jev.",
      }) +
      `<div class="field"><label for="setModel">LLM model</label>` +
      `<div class="row" style="margin-top:0"><select id="setModel" style="flex:1"></select>` +
      `<button id="setFetchModels" type="button" class="ghost">Fetch</button></div>` +
      `<div class="hint">A failed fetch clears the list. Only fetched models can be selected.</div></div>`;
    const fetchNow = () => fetchModelsInto(
      $("setModel"),
      $("setBaseUrl").value.trim(),
      $("setToken").value.trim(),
      d.llm_model || "",
    );
    $("setFetchModels").onclick = fetchNow;
    if (d.llm_base_url && d.llm_token_set) fetchNow();
  } else {
    // Server tab: login password + integration API keys.
    body.innerHTML =
      (d.has_server_key
        ? `<div class="field"><label>Password</label><div class="hint">A password is set. Enter a new one to change it. Leave it empty in a later save to turn login off.</div></div>` +
          fieldHtml({ id: "setServerKey", label: "New password (6+ characters; empty turns login off)", type: "password" })
        : `<div class="field"><label>Password</label><div class="hint">None — anyone can connect. Setting a password turns on the login screen.</div></div>` +
          fieldHtml({ id: "setServerKey", label: "New password (6+ characters)", type: "password" })) +
      `<div class="field"><label>API keys (for apps and scripts, issued and revoked separately)</label>` +
      `<div class="hint">Not the login password, and not the Jev key. Issue one per CI job, agent, or partner. Plaintext is shown once at issue time.</div>` +
      `<div class="row" style="margin-top:8px"><input id="newKeyName" type="text" placeholder="e.g. ci-bot" style="flex:1">` +
      `<button id="issueKey" type="button" class="ghost">Issue</button></div>` +
      `<div id="keyList" style="margin-top:8px"></div></div>`;
    refreshKeyList();
    $("issueKey").onclick = issueKey;
  }
}

async function refreshKeyList() {
  const box = $("keyList");
  if (!box) return;
  box.innerHTML = `<div class="hint">Loading…</div>`;
  try {
    const d = await api("/api/keys");
    const keys = d.keys || [];
    if (!keys.length) { box.innerHTML = `<div class="hint">No keys issued.</div>`; return; }
    box.innerHTML = keys.map((k) =>
      `<div class="row" style="margin-top:6px;justify-content:space-between;align-items:center">` +
      `<span><b>${esc(k.name)}</b> <span class="hint">${esc(k.prefix || "")}… · ${k.revoked ? "revoked" : esc(k.created_at || "")}</span></span>` +
      (k.revoked ? "" : `<button type="button" class="ghost" data-revoke="${k.id}">Revoke</button>`) +
      `</div>`).join("");
    box.querySelectorAll("[data-revoke]").forEach((b) => {
      b.onclick = async () => {
        if (!confirm(`Revoke the '${b.closest(".row").querySelector("b").textContent}' key?`)) return;
        try {
          await api(`/api/keys/${b.dataset.revoke}`, { method: "DELETE" });
          await refreshKeyList();
        } catch (e) { showSetError(e.message); }
      };
    });
  } catch (e) { box.innerHTML = `<div class="hint">${esc(e.message)}</div>`; }
}

async function issueKey() {
  const name = $("newKeyName").value.trim();
  if (!name) { showSetError("Enter a key name (e.g. ci-bot)"); return; }
  try {
    const d = await api("/api/keys", {
      method: "POST", headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name }),
    });
    $("newKeyName").value = "";
    await refreshKeyList();
    const box = $("keyList");
    box.insertAdjacentHTML("afterbegin",
      `<div class="keybox"><code>${esc(d.key)}</code>` +
      `<button type="button" data-copy="${esc(d.key)}">Copy</button></div>` +
      `<div class="hint">${esc(d.warning || "")}</div>`);
    box.querySelector("[data-copy]").onclick = async (e) => {
      try { await navigator.clipboard.writeText(d.key); e.target.textContent = "Copied ✓"; }
      catch { e.target.textContent = "Copy it manually"; }
    };
  } catch (e) { showSetError(e.message); }
}

function readFileAsDataUrl(file) {
  return new Promise((resolve, reject) => {
    if (file.size > 500 * 1024) { reject(Error("file must be 500 KB or smaller")); return; }
    const r = new FileReader();
    r.onload = () => resolve(String(r.result));
    r.onerror = () => reject(Error("could not read the file"));
    r.readAsDataURL(file);
  });
}

async function openSettings() {
  settingsUI.tab = "site";
  settingsUI.clearLogo = false;
  try {
    settingsUI.data = await api("/api/settings");
  } catch (e) {
    if (e.message === "__login_required__") { await requireLogin(auth.status?.site); return; }
    showSetError(e.message);
    return;
  }
  $("settings").classList.remove("hidden");
  renderSettings();
}

function closeSettings() {
  $("settings").classList.add("hidden");
}

async function saveSettings() {
  const patch = {};
  try {
    if (settingsUI.tab === "site") {
      patch.site_name = $("setSiteName").value.trim();
      patch.site_description = $("setSiteDesc").value.trim();
      const file = $("setLogo").files?.[0];
      if (settingsUI.clearLogo) patch.site_logo = null;
      else if (file) patch.site_logo = await readFileAsDataUrl(file);
    } else if (settingsUI.tab === "models") {
      const jev = $("setJevKey")?.value.trim();
      if (jev) patch.jev_api_key = jev;
      patch.llm_base_url = $("setBaseUrl").value.trim();
      if ($("setToken").value.trim()) patch.llm_token = $("setToken").value.trim();
      if ($("setModel").value) patch.llm_model = $("setModel").value;
    } else {
      const v = $("setServerKey")?.value.trim() || "";
      if (v) patch.server_key = v;
    }
    const d = await api("/api/settings", {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(patch),
    });
    settingsUI.data = d.settings;
    settingsUI.clearLogo = false;
    const site = await openApi("/api/site");
    applySite(site);
    renderSettings();
    showSetError("");
    $("stage").textContent = "Settings saved";
  } catch (e) {
    showSetError(e.message === "__login_required__" ? "login required" : e.message);
  }
}

async function bootMain() {
  try {
    const h = await api(`/api/health${rootQuery()}`);
    $("health").textContent = `${h.items ?? 0} items`;
  } catch (e) {
    if (e.message === "__login_required__") { await requireLogin(auth.status?.site); return; }
    $("health").textContent = "Offline";
  }
  try {
    await loadTree();
  } catch (e) {
    if (e.message === "__login_required__") { await requireLogin(auth.status?.site); return; }
    $("empty").textContent = e.message;
    $("empty").classList.remove("hidden");
  }
}

async function boot() {
  wire();
  setMode("search");
  $("settingsBtn").onclick = openSettings;
  $("setClose").onclick = closeSettings;
  document.addEventListener("keydown", (ev) => {
    if (ev.key !== "Escape") return;
    if ($("settings").classList.contains("hidden")) return;
    ev.preventDefault();
    closeSettings();
  });
  $("setSave").onclick = saveSettings;
  $("setLogout").onclick = async () => {
    try { await api("/api/auth/logout", { method: "POST" }); } catch { /* noop */ }
    localStorage.removeItem(TOKEN_KEY);
    $("settings").classList.add("hidden");
    location.reload();
  };
  document.querySelectorAll("#settings .tabs button").forEach((b) => {
    b.onclick = () => { settingsUI.tab = b.dataset.tab; renderSettings(); };
  });
  // Public branding first (login screen needs the name/logo too).
  try {
    const site = await openApi("/api/site");
    applySite(site);
  } catch { /* defaults */ }
  let status;
  try {
    status = await openApi("/api/setup/status");
  } catch {
    $("health").textContent = "Offline";
    return;
  }
  auth.status = status;
  applySite(status.site);
  const hasToken = !!auth.token();
  // Open mode (no password) should load the tree immediately.
  // Only gate on login when a password actually exists.
  if (status.has_server_key && !hasToken) {
    await askLogin(status.site);
    return;
  }
  // Validate a stored token (may be expired/revoked).
  if (hasToken) {
    try {
      await api("/api/health");
    } catch (e) {
      if (e.message === "__login_required__") {
        if (!status.jev_api_key_set) await startOnboard(status);
        else await requireLogin(status.site);
        return;
      }
    }
  }
  await bootMain();
}
boot();
