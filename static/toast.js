class ToastNotification {
  constructor(options = {}) {
    this.position = options.position || "top-right";
    this.duration = options.duration || 5000; // 5 seconds by default
    this.container = null;
    this.createContainer();

    // For debugging to console (optional)
    this.debugToConsole = options.debug || false;
  }

  createContainer() {
    // Check if container already exists
    const existingContainer = document.querySelector(".toast-container");
    if (existingContainer) {
      this.container = existingContainer;
      return;
    }

    this.container = document.createElement("div");
    this.container.className = `toast-container ${this.position}`;
    document.body.appendChild(this.container);

    // Add styles if not already in CSS
    if (!document.getElementById("toast-styles")) {
      const style = document.createElement("style");
      style.id = "toast-styles";
      style.textContent = `
        .toast-container {
          position: fixed;
          z-index: 1000;
          max-width: 350px;
        }
        .toast-container.top-right {
          top: 20px;
          right: 20px;
        }
        .toast-container.top-left {
          top: 20px;
          left: 20px;
        }
        .toast-container.bottom-right {
          bottom: 20px;
          right: 20px;
        }
        .toast-container.bottom-left {
          bottom: 20px;
          left: 20px;
        }
        .toast {
          background-color: #333;
          color: white;
          padding: 15px 25px;
          border-radius: 4px;
          margin-bottom: 10px;
          opacity: 0;
          transform: translateY(-20px);
          transition: all 0.3s ease-in-out;
          box-shadow: 0 2px 10px rgba(0,0,0,0.2);
          position: relative;
        }
        .toast.success {
          background-color: #4CAF50;
        }
        .toast.info {
          background-color: #2196F3;
        }
        .toast.warning {
          background-color: #ff9800;
        }
        .toast.error {
          background-color: #f44336;
        }
        .toast.visible {
          opacity: 1;
          transform: translateY(0);
        }
        .toast-close {
          position: absolute;
          top: 5px;
          right: 5px;
          background: none;
          border: none;
          color: white;
          font-size: 16px;
          cursor: pointer;
          opacity: 0.7;
        }
        .toast-close:hover {
          opacity: 1;
        }
        .toast-title {
          font-weight: bold;
          margin-bottom: 5px;
        }
        .toast-message {
          word-wrap: break-word;
        }
      `;
      document.head.appendChild(style);
    }
  }

  show(message, type = "info", title = "") {
    if (this.debugToConsole) {
      console.log(
        `[${type.toUpperCase()}]${title ? " " + title + ":" : ""} ${message}`,
      );
    }

    // Format object messages
    if (typeof message === "object" && message !== null) {
      message = JSON.stringify(message, null, 2);
    }

    const toast = document.createElement("div");
    toast.className = `toast ${type}`;

    // Create title if provided
    let htmlContent = "";
    if (title) {
      htmlContent += `<div class="toast-title">${title}</div>`;
    }

    // Add message content
    htmlContent += `<div class="toast-message">${message}</div>`;

    // Add close button
    htmlContent += `<button class="toast-close">&times;</button>`;

    toast.innerHTML = htmlContent;

    // Add close button functionality
    toast.querySelector(".toast-close").addEventListener("click", () => {
      this.hide(toast);
    });

    // Add to container
    this.container.appendChild(toast);

    // Trigger animation
    setTimeout(() => toast.classList.add("visible"), 10);

    // Auto-remove after duration
    const timeout = setTimeout(() => {
      this.hide(toast);
    }, this.duration);

    // Store the timeout so we can clear it if needed
    toast.dataset.timeout = timeout;

    return toast;
  }

  hide(toast) {
    // Clear the timeout if it exists
    if (toast.dataset.timeout) {
      clearTimeout(toast.dataset.timeout);
    }

    // Trigger hide animation
    toast.classList.remove("visible");

    // Remove element after animation
    setTimeout(() => {
      if (toast.parentNode === this.container) {
        this.container.removeChild(toast);
      }
    }, 300);
  }

  // Convenience methods for different toast types
  success(message, title = "") {
    return this.show(message, "success", title);
  }

  error(message, title = "") {
    return this.show(message, "error", title);
  }

  info(message, title = "") {
    return this.show(message, "info", title);
  }

  warning(message, title = "") {
    return this.show(message, "warning", title);
  }
}

// Export for global use
window.Toast = new ToastNotification({ debug: true });
