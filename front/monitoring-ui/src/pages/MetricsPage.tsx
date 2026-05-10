import { useEffect, useState } from "react";
import { monitoringApi } from "../api/monitoringApi";
import type { OperationMetricsItem, ServiceMetricsItem } from "../api/types";

export function MetricsPage() {
  const [services, setServices] = useState<ServiceMetricsItem[]>([]);
  const [operations, setOperations] = useState<OperationMetricsItem[]>([]);

  async function load() {
    const [serviceData, operationData] = await Promise.all([
      monitoringApi.getMetricsByService(),
      monitoringApi.getMetricsByOperation(),
    ]);

    setServices(serviceData.items);
    setOperations(operationData.items);
  }

  useEffect(() => {
    void load();
  }, []);

  return (
    <section>
      <div className="page-header">
        <h1>Metrics</h1>
        <button onClick={load}>Refresh</button>
      </div>

      <div className="card table-card">
        <h2>By service</h2>
        <table>
          <thead>
            <tr>
              <th>Service</th>
              <th>Events</th>
              <th>Requests</th>
              <th>Errors</th>
              <th>Error rate</th>
              <th>Avg duration</th>
              <th>Retries</th>
              <th>CB open</th>
            </tr>
          </thead>
          <tbody>
            {services.map((item) => (
              <tr key={item.service}>
                <td>{item.service}</td>
                <td>{item.totalEvents}</td>
                <td>{item.totalRequests}</td>
                <td>{item.totalErrors}</td>
                <td>{item.errorRatePercent.toFixed(2)}%</td>
                <td>{item.avgDurationMs?.toFixed(1) ?? "—"}</td>
                <td>{item.totalRetries}</td>
                <td>{item.totalCircuitBreakerOpen}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="card table-card">
        <h2>By operation</h2>
        <table>
          <thead>
            <tr>
              <th>Service</th>
              <th>Operation</th>
              <th>Requests</th>
              <th>Errors</th>
              <th>Error rate</th>
              <th>Retries</th>
              <th>Idem conflicts</th>
            </tr>
          </thead>
          <tbody>
            {operations.map((item) => (
              <tr key={`${item.service}:${item.operation}`}>
                <td>{item.service}</td>
                <td>{item.operation}</td>
                <td>{item.totalRequests}</td>
                <td>{item.totalErrors}</td>
                <td>{item.errorRatePercent.toFixed(2)}%</td>
                <td>{item.totalRetries}</td>
                <td>{item.totalIdempotencyConflicts}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}