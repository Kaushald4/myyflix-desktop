import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router";
import "./App.css";
import AppRoutes from "./routes";
import QueryProvider from "./providers/QueryProvider";
import { ThemeProvider } from "./providers/ThemeProvider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <ThemeProvider>
    <QueryProvider>
      <BrowserRouter>
        <AppRoutes />
      </BrowserRouter>
    </QueryProvider>
  </ThemeProvider>
);
