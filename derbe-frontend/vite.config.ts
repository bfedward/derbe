import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: { "@": fileURLToPath(new URL("./src", import.meta.url)) },
  },
  server: { port: 4200, strictPort: true },
  build: {
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (!id.includes("node_modules")) return;
          if (id.includes("/react/") || id.includes("/react-dom/")) return "react";
          if (id.includes("/@tanstack/")) return "tanstack";
          if (id.includes("/react-hook-form/") || id.includes("/@hookform/") || id.includes("/zod/")) return "forms";
          if (id.includes("/radix-ui/") || id.includes("/@radix-ui/")) return "radix";
        },
      },
    },
  },
});
