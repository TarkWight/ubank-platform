import type {
  EventListResponse,
  MetricsTimeseriesResponse,
  OperationMetricsResponse,
  OverviewMetricsResponse,
  ServiceMetricsResponse,
} from "./types";

const BASE_URL = import.meta.env.VITE_MONITORING_API_URL ?? "http://localhost:8080";

async function request<T>(path: string): Promise<T> {
  const response = await fetch(`${BASE_URL}${path}`);

  if (!response.ok) {
    const text = await response.text();
    throw new Error(`HTTP ${response.status}: ${text}`);
  }

  return response.json() as Promise<T>;
}

export const monitoringApi = {
  getOverview(): Promise<OverviewMetricsResponse> {
    return request("/api/v1/metrics/overview");
  },

  getTimeseries(bucket: "minute" | "hour" = "minute"): Promise<MetricsTimeseriesResponse> {
    return request(`/api/v1/metrics/timeseries?bucket=${bucket}`);
  },

  getEvents(params: {
    service?: string;
    eventType?: string;
    transport?: string;
    traceId?: string;
    idempotencyKey?: string;
    operation?: string;
    limit?: number;
    offset?: number;
  }): Promise<EventListResponse> {
    const search = new URLSearchParams();

    Object.entries(params).forEach(([key, value]) => {
      if (value !== undefined && value !== null && String(value).trim() !== "") {
        search.set(key, String(value));
      }
    });

    return request(`/api/v1/events?${search.toString()}`);
  },

  getMetricsByService(): Promise<ServiceMetricsResponse> {
    return request("/api/v1/metrics/by-service");
  },

  getMetricsByOperation(): Promise<OperationMetricsResponse> {
    return request("/api/v1/metrics/by-operation");
  },
};