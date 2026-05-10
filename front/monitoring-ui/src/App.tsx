import { Activity, BarChart3, ListTree } from "lucide-react";
import { BrowserRouter, Link, Route, Routes, useLocation } from "react-router-dom";

import { OverviewPage } from "./pages/OverviewPage";
import { EventsPage } from "./pages/EventsPage";
import { MetricsPage } from "./pages/MetricsPage";

import { TracePage } from "./pages/TracePage";
import { IdempotencyPage } from "./pages/IdempotencyPage";

import "./App.css";

function Sidebar() {
  const location = useLocation();

  function isActive(path: string): boolean {
    if (path === "/") {
      return location.pathname === "/";
    }

    return location.pathname.startsWith(path);
  }

  return (
    <aside className="sidebar">
      <div className="brand">
        <Activity size={24} />
        <span>Monitoring</span>
      </div>

      <Link to="/" className={isActive("/") ? "nav-link active" : "nav-link"}>
        <BarChart3 size={18} />
        Overview
      </Link>

      <Link
        to="/events"
        className={isActive("/events") ? "nav-link active" : "nav-link"}
      >
        <ListTree size={18} />
        Events
      </Link>

      <Link
        to="/metrics"
        className={isActive("/metrics") ? "nav-link active" : "nav-link"}
      >
        <BarChart3 size={18} />
        Metrics
      </Link>
    </aside>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <div className="app">
        <Sidebar />

        <main className="content">
          <Routes>
            <Route path="/" element={<OverviewPage />} />
            <Route path="/events" element={<EventsPage />} />
            <Route path="/metrics" element={<MetricsPage />} />

            <Route path="/traces/:traceId" element={<TracePage />} />
            <Route
              path="/idempotency/:idempotencyKey"
              element={<IdempotencyPage />}
            />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}