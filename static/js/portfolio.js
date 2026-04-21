function buildItem(item) {
  const div = document.createElement('div');
  div.className = 'portfolio-item';
  div.dataset.category  = item.category;
  div.dataset.label     = item.label;
  div.dataset.src       = item.src;
  div.dataset.type      = item.media_type;

  if (item.media_type === 'video') {
    const v = document.createElement('video');
    v.src      = item.src;
    v.muted    = true;
    v.loop     = true;
    v.setAttribute('playsinline', '');
    v.setAttribute('autoplay', '');
    v.play().catch(() => {});
    div.appendChild(v);
  } else {
    const img = document.createElement('img');
    img.src = item.src;
    img.alt = item.label;
    div.appendChild(img);
  }

  const overlay = document.createElement('div');
  overlay.className = 'portfolio-overlay';
  overlay.innerHTML = `<span>${item.label}</span>`;
  div.appendChild(overlay);

  return div;
}

function setupFilters(grid) {
  const filterBtns = document.querySelectorAll('.filter-btn');
  if (!filterBtns.length) return;

  filterBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      filterBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      const filter = btn.dataset.filter;
      grid.querySelectorAll('.portfolio-item').forEach(item => {
        item.style.display = filter === 'all' || item.dataset.category === filter ? '' : 'none';
      });
    });
  });
}

function setupLightbox(grid) {
  const lightbox      = document.getElementById('lightbox');
  const lightboxMedia = document.getElementById('lightbox-media');
  const lightboxClose = document.getElementById('lightbox-close');

  grid.querySelectorAll('.portfolio-item').forEach(item => {
    item.addEventListener('click', () => {
      const src  = item.dataset.src;
      const type = item.dataset.type;
      if (!src) return;

      if (type === 'video') {
        lightboxMedia.innerHTML = `<video src="${src}" controls autoplay style="max-width:100%;max-height:80vh;"></video>`;
        lightbox.classList.add('open');
        document.body.style.overflow = 'hidden';
        return;
      }

      lightboxMedia.innerHTML = `<img src="${src}" alt="${item.dataset.label}" />`;
      lightbox.classList.add('open');
      document.body.style.overflow = 'hidden';
    });
  });

  function closeLightbox() {
    const video = lightboxMedia.querySelector('video');
    if (video) video.pause();
    lightbox.classList.remove('open');
    lightboxMedia.innerHTML = '';
    document.body.style.overflow = '';
  }

  lightboxClose?.addEventListener('click', closeLightbox);
  lightbox?.addEventListener('click', e => { if (e.target === lightbox) closeLightbox(); });
  document.addEventListener('keydown', e => { if (e.key === 'Escape') closeLightbox(); });
}

const portfolioGrid = document.getElementById('portfolio-grid');

if (portfolioGrid) {
  fetch('/api/media')
    .then(r => r.json())
    .then(items => {
      items.forEach(item => portfolioGrid.appendChild(buildItem(item)));
      setupFilters(portfolioGrid);
      setupLightbox(portfolioGrid);
    });
} else {
  const featuredGrid = document.getElementById('featured-grid');
  if (featuredGrid) {
    fetch('/api/recent')
      .then(r => r.json())
      .then(items => {
        items.forEach(item => featuredGrid.appendChild(buildItem(item)));
        setupLightbox(featuredGrid);
      });
  }
}
