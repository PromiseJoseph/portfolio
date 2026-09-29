document.addEventListener("DOMContentLoaded", async () => {
  if (!window.particlesJS) return;

  try {
    const response = await fetch("particlejs-config.json");

    if (!response.ok) {
      throw new Error(`Failed to load config: ${response.status}`);
    }

    const particlesConfig = await response.json();

    window.particlesJS("particles-js", particlesConfig);
  } catch (error) {
    console.error("Failed to load particles configuration:", error);
  }

  document.querySelectorAll("button").forEach((button) => {
    button.addEventListener("click", (event) => {
      const targetId = event.target.classList.contains("projects-button")
        ? "projects"
        : event.target.classList.contains("op-button")
          ? "top"
          : event.target.classList.contains("contact-button")
            ? "footer"
            : null;

      if (targetId) {
        const targetElement = document.getElementById(targetId);
        if (targetElement) {
          targetElement.scrollIntoView({ behavior: "smooth" });
        }
      }
    });
  });
});
