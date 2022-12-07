
export interface Page<T, K> {
  next: K
  items: T[]
  total: number | null
}
