import { useState } from "react";
import { Activity, BarChart3, ListTree } from "lucide-react";
import { OverviewPage } from "./pages/OverviewPage";
import { EventsPage } from "./pages/EventsPage";
import { MetricsPage } from "./pages/MetricsPage";
import "./App.css";

type Page = "overview" | "events" | "metrics";

export default function App() {
  const [page, setPage] = useState<Page>("overview");

  return (
    <div className="app">
      <aside className="sidebar">
        <div className="brand">
          <Activity size={24} />
          <span>Monitoring</span>
        </div>

        <button className={page === "overview" ? "active" : ""} onClick={() => setPage("overview")}>
          <BarChart3 size={18} />
          Overview
        </button>

        <button className={page === "events" ? "active" : ""} onClick={() => setPage("events")}>
          <ListTree size={18} />
          Events
        </button>

        <button className={page === "metrics" ? "active" : ""} onClick={() => setPage("metrics")}>
          <BarChart3 size={18} />
          Metrics
        </button>
      </aside>

      <main className="content">
        {page === "overview" && <OverviewPage />}
        {page === "events" && <EventsPage />}
        {page === "metrics" && <MetricsPage />}
      </main>
    </div>
  );
}