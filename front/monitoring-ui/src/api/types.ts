export type OverviewMetricsResponse = {
  totalEvents: number;
  totalRequests: number;
  totalErrors: number;
  errorRatePercent: number;
  avgDurationMs: number | null;
  totalRetries: number;
  totalCircuitBreakerOpen: number;
  totalIdempotencyReplays: number;
  totalIdempotencyInProgress: number;
  totalIdempotencyConflicts: number;
};

export type MetricsTimeseriesPoint = {
  bucketStart: string;
  totalEvents: number;
  totalRequests: number;
  totalErrors: number;
  errorRatePercent: number;
  avgDurationMs: number | null;
  totalRetries: number;
  totalCircuitBreakerOpen: number;
  totalIdempotencyReplays: number;
  totalIdempotencyInProgress: number;
  totalIdempotencyConflicts: number;
};

export type MetricsTimeseriesResponse = {
  items: MetricsTimeseriesPoint[];
};

export type TraceError = {
  code: string | null;
  type: string | null;
  message: string | null;
};

export type EventItem = {
  id: number;
  traceId: string;
  idempotencyKey: string | null;
  timestamp: string;
  service: string;
  transport: "HTTP" | "WS" | null;
  operation: string | null;
  eventType: string;
  spanId: string | null;
  parentSpanId: string | null;
  method: string | null;
  path: string | null;
  status: number | null;
  durationMs: number | null;
  success: boolean | null;
  attempt: number | null;
  error: TraceError | null;
};

export type EventListResponse = {
  items: EventItem[];
  limit: number;
  offset: number;
};

export type ServiceMetricsItem = {
  service: string;
  totalEvents: number;
  totalRequests: number;
  totalErrors: number;
  errorRatePercent: number;
  avgDurationMs: number | null;
  totalRetries: number;
  totalCircuitBreakerOpen: number;
  totalIdempotencyReplays: number;
  totalIdempotencyInProgress: number;
  totalIdempotencyConflicts: number;
};

export type ServiceMetricsResponse = {
  items: ServiceMetricsItem[];
};

export type OperationMetricsItem = ServiceMetricsItem & {
  operation: string;
};

export type OperationMetricsResponse = {
  items: OperationMetricsItem[];
};