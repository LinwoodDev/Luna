/**
 * Fetch with Cache API + conditional GET, but
 * fall back to a plain fetch() if anything blows up.
 *
 * @param {string|Request} input
 * @param {string} cacheName
 * @returns {Promise<Response>}
 */
async function fetchCached(input, cacheName = "fetch-cache") {
  // If Cache API isn’t there, or you’re not on HTTPS/localhost, skip smart logic
  if (!("caches" in window)) {
    console.warn("Cache API not available, falling back to plain fetch");
    return fetch(input);
  }

  try {
    const req = input instanceof Request ? input.clone() : new Request(input);
    const cache = await caches.open(cacheName);
    const cached = await cache.match(req);

    // Build conditional headers
    const headers = new Headers();
    if (cached) {
      const etag = cached.headers.get("ETag");
      const lastMod = cached.headers.get("Last-Modified");
      if (etag) headers.set("If-None-Match", etag);
      if (lastMod) headers.set("If-Modified-Since", lastMod);
    }

    // One round‑trip: either 304 or fresh body
    const networkResponse = await fetch(req, { headers });
    if (networkResponse.status === 304 && cached) {
      console.log("fetchCached:", input, "-> 304, returning cached response");
      return cached.clone();
    }

    // On 200–299, update cache
    if (networkResponse.ok) {
      cache.put(req, networkResponse.clone()).catch(() => {
        /* swallow */
      });
    }
    console.log("fetchCached:", input, "->", networkResponse.status);
    return networkResponse;
  } catch (err) {
    console.warn("fetchCached failed, falling back to plain fetch", err);
    return fetch(input);
  }
}

function renderCategories(allCategories, included, excluded, onUpdate) {
  const container = document.getElementById("search-categories");
  if (!container) return;

  container.innerHTML = "";
  allCategories.forEach((cat) => {
    const row = document.createElement("div");
    row.className = "row gap-xs align-center p-xs";
    row.style.backgroundColor = "var(--ls-elevation)";
    row.style.borderRadius = "0.5rem";

    const name = document.createElement("span");
    name.textContent = cat;
    name.style.flex = "1";

    const includeBtn = document.createElement("button");
    includeBtn.type = "button";
    includeBtn.className = "btn";
    includeBtn.style.padding = "2px 8px";
    includeBtn.textContent = "+";
    includeBtn.title = "Include category";

    const excludeBtn = document.createElement("button");
    excludeBtn.type = "button";
    excludeBtn.className = "btn";
    excludeBtn.style.padding = "2px 8px";
    excludeBtn.textContent = "-";
    excludeBtn.title = "Exclude category";

    const includeInput = document.createElement("input");
    includeInput.type = "hidden";
    includeInput.name = "include_category";
    includeInput.value = cat;
    includeInput.disabled = !included.includes(cat);

    const excludeInput = document.createElement("input");
    excludeInput.type = "hidden";
    excludeInput.name = "exclude_category";
    excludeInput.value = cat;
    excludeInput.disabled = !excluded.includes(cat);

    const updateState = () => {
      if (!includeInput.disabled) {
        includeBtn.classList.add("primary");
      } else {
        includeBtn.classList.remove("primary");
      }

      if (!excludeInput.disabled) {
        excludeBtn.classList.add("primary");
        excludeBtn.style.backgroundColor = "#ff4d4d";
        excludeBtn.style.color = "white";
      } else {
        excludeBtn.classList.remove("primary");
        excludeBtn.style.backgroundColor = "";
        excludeBtn.style.color = "";
      }
    };

    updateState();

    includeBtn.onclick = () => {
      if (includeInput.disabled) {
        includeInput.disabled = false;
        excludeInput.disabled = true;
      } else {
        includeInput.disabled = true;
      }
      updateState();
      if (onUpdate) onUpdate();
    };

    excludeBtn.onclick = () => {
      if (excludeInput.disabled) {
        excludeInput.disabled = false;
        includeInput.disabled = true;
      } else {
        excludeInput.disabled = true;
      }
      updateState();
      if (onUpdate) onUpdate();
    };

    row.appendChild(name);
    row.appendChild(includeBtn);
    row.appendChild(excludeBtn);
    row.appendChild(includeInput);
    row.appendChild(excludeInput);
    container.appendChild(row);
  });

  const filtersEl = document.getElementById("search-filters");
  if (filtersEl) {
    filtersEl.style.display = "block";
  }
}

async function initSearch() {
  const params = new URLSearchParams(window.location.search);
  const resultsSection = document.getElementById("search-results");
  const statusEl = document.getElementById("search-status");
  const listEl = document.getElementById("search-list");
  const template = document.getElementById("search-item-template");
  const emptySection = document.getElementById("search-empty");

  const hasParams = Array.from(params.keys()).length > 0;
  const includeAssets = !hasParams || params.get("assets") === "on";
  const includeAuthors = !hasParams || params.get("authors") === "on";

  const includedCategories = params.getAll("include_category");
  const excludedCategories = params.getAll("exclude_category");

  // keep form inputs filled after submit
  const searchInput = document.getElementById("search-input");
  if (searchInput) {
    searchInput.value = params.get("q") || "";
    searchInput.addEventListener("input", performSearch);
  }
  const searchAuthors = document.getElementById("search-authors");
  if (searchAuthors) {
    searchAuthors.checked = includeAuthors;
    searchAuthors.addEventListener("change", performSearch);
  }
  const searchAssets = document.getElementById("search-assets");
  if (searchAssets) {
    searchAssets.checked = includeAssets;
    searchAssets.addEventListener("change", performSearch);
  }

  let assetsData = [];
  let authorsData = [];

  function performSearch() {
    const query = document
      .getElementById("search-input")
      .value.trim()
      .toLowerCase();
    const includeAssets = document.getElementById("search-assets").checked;
    const includeAuthors = document.getElementById("search-authors").checked;

    const includedCategories = Array.from(
      document.querySelectorAll('input[name="include_category"]:not(:disabled)')
    ).map((el) => el.value);
    const excludedCategories = Array.from(
      document.querySelectorAll('input[name="exclude_category"]:not(:disabled)')
    ).map((el) => el.value);

    // Update URL
    const url = new URL(window.location);
    if (query) url.searchParams.set("q", query);
    else url.searchParams.delete("q");
    url.searchParams.set("assets", includeAssets ? "on" : "off");
    url.searchParams.set("authors", includeAuthors ? "on" : "off");
    url.searchParams.delete("include_category");
    url.searchParams.delete("exclude_category");
    includedCategories.forEach((c) =>
      url.searchParams.append("include_category", c)
    );
    excludedCategories.forEach((c) =>
      url.searchParams.append("exclude_category", c)
    );
    window.history.replaceState({}, "", url);

    if (
      !query &&
      includedCategories.length === 0 &&
      excludedCategories.length === 0
    ) {
      // If no query and no filters, show all results (or handle as desired)
      // For now, we proceed to show all results.
      // If you want to show empty state instead, uncomment the lines below:
      /*
        statusEl.textContent = '';
        resultsSection.style.display = 'none';
        if (emptySection) emptySection.style.display = 'block';
        return;
        */
    }

    if (emptySection) emptySection.style.display = "none";
    resultsSection.style.display = "block";

    const assetMatches = assetsData.filter((item) => {
      const itemCats = item.categories || [];

      if (excludedCategories.some((cat) => itemCats.includes(cat))) {
        return false;
      }

      if (includedCategories.length > 0) {
        if (!includedCategories.some((cat) => itemCats.includes(cat))) {
          return false;
        }
      }

      if (!query) return true;

      const hay = [item.name, item.description || "", ...itemCats]
        .join(" ")
        .toLowerCase();
      return hay.includes(query);
    });

    const authorMatches = authorsData.filter((author) => {
      if (!query) return true;
      return (
        author.name.toLowerCase().includes(query) ||
        author.display_name?.toLowerCase().includes(query)
      );
    });

    statusEl.textContent = "";
    statusEl.style.display = "none";
    listEl.innerHTML = "";

    if (includeAssets && assetMatches.length) {
      const h2 = document.createElement("h3");
      h2.textContent = "Assets";
      listEl.appendChild(h2);
      assetMatches.forEach((item) => {
        const node = template.cloneNode(true);
        node.id = "";
        node.style.display = "";
        node.querySelector(".search-link").href =
          encodeURIComponent(item.author || "unknown") +
          "/" +
          encodeURIComponent(item.name);
        node.querySelector(".search-title").textContent = item.name;
        node.querySelector(".search-description").textContent = item.author
          ? `by ${item.author}`
          : "";
        listEl.appendChild(node);
      });
    }

    if (includeAuthors && authorMatches.length) {
      const h2 = document.createElement("h3");
      h2.textContent = "Authors";
      listEl.appendChild(h2);
      authorMatches.forEach((author) => {
        const node = template.cloneNode(true);
        node.id = "";
        node.style.display = "";
        node.querySelector(".search-link").href = encodeURIComponent(
          author.name
        );
        node.querySelector(".search-title").textContent =
          author.display_name || author.name;
        node.querySelector(".search-description").textContent =
          author.email || "";
        listEl.appendChild(node);
      });
    }

    const hasResults = (includeAssets && assetMatches.length > 0) || (includeAuthors && authorMatches.length > 0);

    if (!hasResults) {
      resultsSection.style.display = 'none';
      if (emptySection) {
        emptySection.style.display = 'block';
        const h3 = emptySection.querySelector('h3');
        if (h3) h3.textContent = 'No results found';
        const p = emptySection.querySelector('p');
        if (p) p.textContent = 'Try different keywords or filters.';
      }
    } else {
      statusEl.style.display = "none";
    }
  }

  try {
    const res = await fetchCached("index.json");
    const data = await res.json();
    assetsData = data.assets || [];
    authorsData = data.authors || [];

    // Extract all unique categories
    const allCategories = new Set();
    assetsData.forEach((asset) => {
      if (asset.categories) {
        asset.categories.forEach((c) => allCategories.add(c));
      }
    });
    renderCategories(
      Array.from(allCategories).sort(),
      includedCategories,
      excludedCategories,
      performSearch
    );

    performSearch();
  } catch (err) {
    console.error(err);
    statusEl.textContent = "Error loading data.";
  } finally {
    // resultsSection.style.display = 'block'; // Handled in performSearch
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initSearch);
} else {
  initSearch();
}
