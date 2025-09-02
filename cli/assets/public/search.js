
/**
 * Fetch with Cache API + conditional GET, but
 * fall back to a plain fetch() if anything blows up.
 *
 * @param {string|Request} input
 * @param {string} cacheName
 * @returns {Promise<Response>}
 */
async function fetchCached(input, cacheName = 'fetch-cache') {
  // If Cache API isn’t there, or you’re not on HTTPS/localhost, skip smart logic
  if (!('caches' in window)) {
    console.warn('Cache API not available, falling back to plain fetch');
    return fetch(input);
  }

  try {
    const req  = input instanceof Request ? input.clone() : new Request(input);
    const cache = await caches.open(cacheName);
    const cached = await cache.match(req);

    // Build conditional headers
    const headers = new Headers();
    if (cached) {
      const etag    = cached.headers.get('ETag');
      const lastMod = cached.headers.get('Last-Modified');
      if (etag)     headers.set('If-None-Match', etag);
      if (lastMod)  headers.set('If-Modified-Since', lastMod);
    }

    // One round‑trip: either 304 or fresh body
    const networkResponse = await fetch(req, { headers });
    if (networkResponse.status === 304 && cached) {
      console.log('fetchCached:', input, '-> 304, returning cached response');
      return cached.clone();
    }

    // On 200–299, update cache
    if (networkResponse.ok) {
      cache.put(req, networkResponse.clone()).catch(() => {/* swallow */});
    }
    console.log('fetchCached:', input, '->', networkResponse.status);
    return networkResponse;
  } catch (err) {
    console.warn('fetchCached failed, falling back to plain fetch', err);
    return fetch(input);
  }
}

document.addEventListener('DOMContentLoaded', async () => {
  const params = new URLSearchParams(window.location.search);
  const query = (params.get('q') || '').trim().toLowerCase();
  const resultsSection = document.getElementById('search-results');
  const statusEl = document.getElementById('search-status');
  const listEl = document.getElementById('search-list');
  const template = document.getElementById('search-item-template');

  const includeAssets = params.get('assets') === 'on';
  const includeAuthors = params.get('authors') === 'on';

  // keep form inputs filled after submit
  document.getElementById('search-input').value = params.get('q') || '';
  if (!query) {
    statusEl.textContent = 'Please enter a search term.';
    resultsSection.style.display = 'block';
    return;
  }
  document.getElementById('search-authors').checked = includeAuthors;
  document.getElementById('search-assets').checked = includeAssets;

  try {
    const res = await fetchCached('index.json');
    const data = await res.json();
    const { assets = [], authors = [] } = data;
    const assetMatches = assets.filter(item => {
      const hay = [item.name, item.description || '', ...(item.tags || [])].join(' ').toLowerCase();
      return hay.includes(query);
    });
    const authorMatches = authors.filter(author => author.name.toLowerCase().includes(query) || author.display_name?.toLowerCase().includes(query));

    statusEl.textContent = '';
    statusEl.style.display = 'none';
    listEl.innerHTML = '';

    if (includeAssets && assetMatches.length) {
      const h2 = document.createElement('h3'); h2.textContent = 'Assets';
      listEl.appendChild(h2);
      assetMatches.forEach(item => {
        const node = template.cloneNode(true);
        node.id = '';
        node.style.display = '';
        node.querySelector('.search-link').href = encodeURIComponent(item.author || 'unknown') + '/' + encodeURIComponent(item.name);
        node.querySelector('.search-title').textContent = item.name;
        node.querySelector(".search-description").textContent = item.author ? `by ${item.author}` : '';
        listEl.appendChild(node);
      });
    }

    if (includeAuthors && authorMatches.length) {
      const h2 = document.createElement('h3'); h2.textContent = 'Authors';
      listEl.appendChild(h2);
      authorMatches.forEach(author => {
        const node = template.cloneNode(true);
        node.id = '';
        node.style.display = '';
        node.querySelector('.search-link').href = encodeURIComponent(author.name);
        node.querySelector('.search-title').textContent = author.display_name || author.name;
        node.querySelector('.search-description').textContent = author.email || '';
        listEl.appendChild(node);
      });
    }

    if (!assetMatches.length && !authorMatches.length) {
      statusEl.textContent = 'No results found.';
    } else {
      statusEl.style.display = 'none';
    }
  } catch (err) {
    console.error(err);
    statusEl.textContent = 'Error loading data.';
  } finally {
    resultsSection.style.display = 'block';
  }
});