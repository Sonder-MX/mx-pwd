export interface Resp<T = any> {
  code: number
  msg?: string
  data?: T
}
