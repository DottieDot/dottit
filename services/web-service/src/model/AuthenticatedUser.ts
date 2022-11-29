
export interface AuthenticatedUser<U> {
  apiToken: string,
  user: U
}
