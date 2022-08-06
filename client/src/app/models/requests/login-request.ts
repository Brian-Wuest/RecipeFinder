/**
 * This model represents a login request.
 */
export interface ILoginRequest {
  /**
   * The name or e-mail address of the user to log in.
   */
  name: string;

  /**
   * The password to verify against the registered password.
   */
  password: string;
}
