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
    const res = await fetch('index.json');
    const data = await res.json();
    const { assets = [], authors = [] } = data;
    const assetMatches = assets.filter(item => {
      const hay = [item.name, item.description || '', ...(item.tags || [])].join(' ').toLowerCase();
      return hay.includes(query);
    });
    const authorMatches = authors.filter(author => author.name.toLowerCase().includes(query));

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
        node.querySelector('.search-title').textContent = author.name;
        node.querySelector('.search-description').textContent = author.email || '';
        listEl.appendChild(node);
      });
    }

    if (!assetMatches.length && !authorMatches.length) {
      statusEl.textContent = 'No results found.';
    }
  } catch (err) {
    console.error(err);
    statusEl.textContent = 'Error loading data.';
  } finally {
    resultsSection.style.display = 'block';
  }
});