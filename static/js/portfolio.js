// ── Filter ──
const filterBtns = document.querySelectorAll('.filter-btn');
const items      = document.querySelectorAll('.portfolio-item');

filterBtns.forEach(btn => {
  btn.addEventListener('click', () => {
    filterBtns.forEach(b => b.classList.remove('active'));
    btn.classList.add('active');

    const filter = btn.dataset.filter;
    items.forEach(item => {
      const show = filter === 'all' || item.dataset.category === filter;
      item.style.display = show ? '' : 'none';
    });
  });
});

// ── Lightbox ──
const lightbox      = document.getElementById('lightbox');
const lightboxMedia = document.getElementById('lightbox-media');
const lightboxClose = document.getElementById('lightbox-close');

items.forEach(item => {
  item.addEventListener('click', () => {
    const src  = item.dataset.src;
    const type = item.dataset.type;
    if (!src) return; // placeholder — no media yet

    lightboxMedia.innerHTML = type === 'video'
      ? `<video src="${src}" controls autoplay style="max-width:90vw;max-height:88vh;border-radius:4px;"></video>`
      : `<img src="${src}" alt="${item.dataset.label}" />`;

    lightbox.classList.add('open');
    document.body.style.overflow = 'hidden';
  });
});

function closeLightbox() {
  lightbox.classList.remove('open');
  lightboxMedia.innerHTML = '';
  document.body.style.overflow = '';
}

lightboxClose.addEventListener('click', closeLightbox);
lightbox.addEventListener('click', e => { if (e.target === lightbox) closeLightbox(); });
document.addEventListener('keydown', e => { if (e.key === 'Escape') closeLightbox(); });
