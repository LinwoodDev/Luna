(() => {
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
      return cached.clone();
    }

    // On 200–299, update cache
    if (networkResponse.ok) {
      cache.put(req, networkResponse.clone()).catch(() => {
        /* swallow */
      });
    }
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
    row.className = "row align-center gap-xs";

    const name = document.createElement("span");
    name.textContent = cat;
    name.className = "flex";

    const includeBtn = document.createElement("button");
    includeBtn.type = "button";
    includeBtn.className = "btn secondary";
    includeBtn.textContent = "Include";
    includeBtn.title = `Include ${cat}`;
    includeBtn.setAttribute("aria-label", `Include ${cat}`);

    const excludeBtn = document.createElement("button");
    excludeBtn.type = "button";
    excludeBtn.className = "btn secondary";
    excludeBtn.textContent = "Exclude";
    excludeBtn.title = `Exclude ${cat}`;
    excludeBtn.setAttribute("aria-label", `Exclude ${cat}`);

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
        includeBtn.classList.add("active");
      } else {
        includeBtn.classList.remove("active");
      }
      includeBtn.setAttribute("aria-pressed", String(!includeInput.disabled));

      if (!excludeInput.disabled) {
        excludeBtn.classList.add("active");
      } else {
        excludeBtn.classList.remove("active");
      }
      excludeBtn.setAttribute("aria-pressed", String(!excludeInput.disabled));
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
    filtersEl.hidden = false;
  }
}

function populateResult(node, href, title, description) {
  const link = node.querySelector(".search-link");
  const titleElement = node.querySelector(".search-title");
  if (!link || !titleElement) return false;

  link.href = href;
  titleElement.textContent = title;

  let descriptionElement = node.querySelector(".search-description");
  if (!descriptionElement && description) {
    descriptionElement = document.createElement("span");
    descriptionElement.className = "search-description";
    titleElement.parentElement?.appendChild(descriptionElement);
  }
  if (descriptionElement) {
    descriptionElement.textContent = description;
    descriptionElement.hidden = !description;
  }
  return true;
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
  let searchTimer;
  const scheduleSearch = () => {
    window.clearTimeout(searchTimer);
    searchTimer = window.setTimeout(performSearch, 120);
  };

  if (searchInput) {
    searchInput.value = params.get("q") || "";
    searchInput.addEventListener("input", scheduleSearch);
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

    if (emptySection) emptySection.hidden = true;
    resultsSection.hidden = false;

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

      const hay = [item.name, item.summary || "", item.description || "", ...itemCats]
        .join(" ")
        .toLowerCase();
      return hay.includes(query);
    });

    const authorMatches = authorsData.filter((author) => {
      if (!query) return true;
      return [author.name, author.display_name || "", author.description || ""]
        .join(" ")
        .toLowerCase()
        .includes(query);
    });

    statusEl.textContent = "";
    statusEl.style.display = "none";
    listEl.innerHTML = "";

    if (includeAssets && assetMatches.length) {
      const heading = document.createElement("li");
      heading.className = "h3 bold";
      heading.textContent = "Assets";
      listEl.appendChild(heading);
      assetMatches.forEach((item) => {
        const node = template.cloneNode(true);
        node.id = "";
        node.style.display = "";
        const href =
          encodeURIComponent(item.author || "unknown") +
          "/" +
          encodeURIComponent(item.name);
        if (populateResult(node, href, item.name, item.author ? `by ${item.author}` : "")) {
          listEl.appendChild(node);
        }
      });
    }

    if (includeAuthors && authorMatches.length) {
      const heading = document.createElement("li");
      heading.className = "h3 bold";
      heading.textContent = "Authors";
      listEl.appendChild(heading);
      authorMatches.forEach((author) => {
        const node = template.cloneNode(true);
        node.id = "";
        node.style.display = "";
        if (populateResult(
          node,
          encodeURIComponent(author.name),
          author.display_name || author.name,
          author.email || "",
        )) {
          listEl.appendChild(node);
        }
      });
    }

    const hasResults = (includeAssets && assetMatches.length > 0) || (includeAuthors && authorMatches.length > 0);

    if (!hasResults) {
      resultsSection.hidden = true;
      if (emptySection) {
        emptySection.hidden = false;
        const h3 = emptySection.querySelector('h3');
        if (h3) h3.textContent = 'No results found';
        const p = emptySection.querySelector('p');
        if (p) p.textContent = 'Try different keywords or filters.';
      }
    } else {
      const resultCount =
        (includeAssets ? assetMatches.length : 0) +
        (includeAuthors ? authorMatches.length : 0);
      statusEl.textContent = `${resultCount} ${resultCount === 1 ? "result" : "results"}`;
      statusEl.style.display = "block";
    }
  }

  try {
    const res = await fetchCached("index.json");
    if (!res.ok) throw new Error(`The registry returned HTTP ${res.status}.`);
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
    resultsSection.hidden = false;
    statusEl.style.display = "block";
    statusEl.textContent = "The registry could not be loaded. Please try again.";
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initSearch);
} else {
  initSearch();
}
})();
