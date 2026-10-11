document.addEventListener("DOMContentLoaded", () => {
  const widthSlider = document.getElementById("width-slider");
  const gapSlider = document.getElementById("gap-slider");
  const stickyCheckbox = document.getElementById("sticky-checkbox");
  const snapToggle = document.getElementById("snap-toggle");
  const prevBtn = document.getElementById("prev-btn");
  const nextBtn = document.getElementById("next-btn");
  const helpBtn = document.getElementById("help-btn");
  const helpModal = document.getElementById("help-modal");
  const helpClose = document.getElementById("help-close");
  const themeBtn = document.getElementById("theme-btn");
  const progressBar = document.getElementById("progress-bar");
  const currentIndexDisplay = document.getElementById("current-index");
  const activeFilenameDisplay = document.getElementById("active-filename");

  let activeIndex = 0;
  let isSnapMode = false;

  // Initialize from local storage or defaults
  const savedWidth = localStorage.getItem("scroller-width") || "100";
  const savedGap = localStorage.getItem("scroller-gap") || "40";
  const savedSnap = localStorage.getItem("scroller-snap") === "true";
  const savedTheme = localStorage.getItem("scroller-theme") || "dark";
  const savedSticky = localStorage.getItem("scroller-sticky") === "true";

  widthSlider.value = savedWidth;
  document.documentElement.style.setProperty(
    "--img-max-width",
    savedWidth + "%",
  );

  gapSlider.value = savedGap;
  document.documentElement.style.setProperty("--img-gap", savedGap + "px");

  if (savedSnap) {
    enableSnapMode();
  }

  if (savedTheme === "light") {
    document.body.classList.add("light-theme");
  }

  if (savedSticky) {
    stickyCheckbox.checked = true;
    document.body.classList.add("sticky-header-active");
  } else {
    stickyCheckbox.checked = false;
    document.body.classList.remove("sticky-header-active");
  }

  // Handle Width
  widthSlider.addEventListener("input", (e) => {
    const val = e.target.value;
    document.documentElement.style.setProperty("--img-max-width", val + "%");
    localStorage.setItem("scroller-width", val);
  });

  // Handle Gap
  gapSlider.addEventListener("input", (e) => {
    const val = e.target.value;
    document.documentElement.style.setProperty("--img-gap", val + "px");
    localStorage.setItem("scroller-gap", val);
  });

  // Handle Sticky Header checkbox
  stickyCheckbox.addEventListener("change", () => {
    if (stickyCheckbox.checked) {
      document.body.classList.add("sticky-header-active");
      localStorage.setItem("scroller-sticky", "true");
    } else {
      document.body.classList.remove("sticky-header-active");
      localStorage.setItem("scroller-sticky", "false");
    }
  });

  // Theme
  themeBtn.addEventListener("click", () => {
    document.body.classList.toggle("light-theme");
    const isLight = document.body.classList.contains("light-theme");
    localStorage.setItem("scroller-theme", isLight ? "light" : "dark");
  });

  // Snap
  function enableSnapMode() {
    document.body.classList.add("snap-mode");
    snapToggle.classList.add("active");
    isSnapMode = true;
    localStorage.setItem("scroller-snap", "true");
  }

  function disableSnapMode() {
    document.body.classList.remove("snap-mode");
    snapToggle.classList.remove("active");
    isSnapMode = false;
    localStorage.setItem("scroller-snap", "false");
  }

  snapToggle.addEventListener("click", () => {
    if (isSnapMode) disableSnapMode();
    else enableSnapMode();
  });

  // Help Modal
  helpBtn.addEventListener("click", (e) => {
    e.stopPropagation();
    helpModal.style.display =
      helpModal.style.display === "block" ? "none" : "block";
  });
  helpClose.addEventListener("click", () => {
    helpModal.style.display = "none";
  });
  document.addEventListener("click", (e) => {
    if (
      helpModal.style.display === "block" &&
      !helpModal.contains(e.target) &&
      e.target !== helpBtn
    ) {
      helpModal.style.display = "none";
    }
  });

  // Navigation
  const cards = document.querySelectorAll(".image-card");

  function scrollToImage(index) {
    if (index >= 0 && index < cards.length) {
      cards[index].scrollIntoView({ behavior: "smooth", block: "start" });
      activeIndex = index;
      updateActiveState(index);
    }
  }

  prevBtn.addEventListener("click", () => {
    scrollToImage(activeIndex - 1);
  });

  nextBtn.addEventListener("click", () => {
    scrollToImage(activeIndex + 1);
  });

  // Intersection Observer
  const observerOptions = {
    root: null,
    rootMargin: "-30% 0px -40% 0px",
    threshold: 0,
  };

  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        const index = parseInt(entry.target.getAttribute("data-index"));
        activeIndex = index;
        updateActiveState(index);
      }
    });
  }, observerOptions);

  cards.forEach((card) => observer.observe(card));

  function updateActiveState(index) {
    cards.forEach((card, idx) => {
      if (idx === index) {
        card.classList.add("active");
      } else {
        card.classList.remove("active");
      }
    });
    currentIndexDisplay.textContent = index + 1;
    const filename = images[index];
    activeFilenameDisplay.textContent = filename;
  }

  // Scroll Progress
  window.addEventListener("scroll", () => {
    const winScroll =
      document.documentElement.scrollTop || document.body.scrollTop;
    const height =
      document.documentElement.scrollHeight -
      document.documentElement.clientHeight;
    const scrolled = height > 0 ? (winScroll / height) * 100 : 0;
    progressBar.style.width = scrolled + "%";
  });

  // Keyboard
  document.addEventListener("keydown", (e) => {
    if (e.key === "ArrowDown" || e.key === "j" || e.key === "J") {
      e.preventDefault();
      scrollToImage(activeIndex + 1);
    } else if (e.key === "ArrowUp" || e.key === "k" || e.key === "K") {
      e.preventDefault();
      scrollToImage(activeIndex - 1);
    } else if (e.key === " " || e.key === "Spacebar") {
      e.preventDefault();
      scrollToImage(activeIndex + 1);
    } else if (
      e.key === "ArrowLeft" ||
      e.key === "h" ||
      e.key === "H" ||
      e.key === "a" ||
      e.key === "A"
    ) {
      e.preventDefault();
      let nextVal = Math.max(30, parseInt(widthSlider.value) - 2);
      widthSlider.value = nextVal;
      widthSlider.dispatchEvent(new Event("input"));
    } else if (
      e.key === "ArrowRight" ||
      e.key === "l" ||
      e.key === "L" ||
      e.key === "d" ||
      e.key === "D"
    ) {
      e.preventDefault();
      let nextVal = Math.min(100, parseInt(widthSlider.value) + 2);
      widthSlider.value = nextVal;
      widthSlider.dispatchEvent(new Event("input"));
    } else if (e.key === "w" || e.key === "W") {
      e.preventDefault();
      window.scrollBy(0, -60);
    } else if (e.key === "s" || e.key === "S") {
      e.preventDefault();
      window.scrollBy(0, 60);
    } else if (e.key === "Home") {
      e.preventDefault();
      scrollToImage(0);
    } else if (e.key === "End") {
      e.preventDefault();
      scrollToImage(cards.length - 1);
    } else if (e.key === "n" || e.key === "N") {
      snapToggle.click();
    } else if (e.key === "t" || e.key === "T") {
      themeBtn.click();
    } else if (e.key === "?" || e.key === "/") {
      helpBtn.click();
    } else if (e.key === "Escape") {
      helpModal.style.display = "none";
      if (document.fullscreenElement) {
        document.exitFullscreen();
      }
    } else if (e.key === "0") {
      widthSlider.value = 100;
      widthSlider.dispatchEvent(new Event("input"));
    } else if (e.key === "f" || e.key === "F") {
      const activeCard = cards[activeIndex];
      if (activeCard) {
        const img = activeCard.querySelector("img");
        if (!document.fullscreenElement) {
          img.requestFullscreen().catch((err) => {
            console.error(`Error fullscreen: ${err.message}`);
          });
        } else {
          document.exitFullscreen();
        }
      }
    }
  });

  window.updateImageInfo = function (img, index) {
    const sizeSpan = document.getElementById(`meta-size-${index}`);
    if (sizeSpan && img.naturalWidth && img.naturalHeight) {
      const w = img.naturalWidth;
      const h = img.naturalHeight;
      const aspect = (w / h).toFixed(2);
      sizeSpan.textContent = `${w} × ${h} (${aspect})`;
    }
  };

  window.copyText = function (text, btnId) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text).then(() => showFeedback(btnId));
    } else {
      const textarea = document.createElement("textarea");
      textarea.value = text;
      textarea.style.position = "fixed";
      textarea.style.opacity = "0";
      document.body.appendChild(textarea);
      textarea.select();
      try {
        document.execCommand("copy");
        showFeedback(btnId);
      } catch (err) {
        console.error("Fallback copy failed", err);
      }
      document.body.removeChild(textarea);
    }
  };

  function showFeedback(btnId) {
    const btn = document.getElementById(btnId);
    const originalText = btn.innerHTML;
    btn.innerHTML = "Copied!";
    btn.classList.add("copied");
    setTimeout(() => {
      btn.innerHTML = originalText;
      btn.classList.remove("copied");
    }, 1200);
  }

  window.copyRelPath = function (idx) {
    const btn = document.getElementById(`copy-rel-btn-${idx}`);
    const text = btn.getAttribute("data-relative");
    copyText(text, `copy-rel-btn-${idx}`);
  };

  window.copyAbsPath = function (idx) {
    const btn = document.getElementById(`copy-abs-btn-${idx}`);
    const text = btn.getAttribute("data-absolute");
    copyText(text, `copy-abs-btn-${idx}`);
  };
});
