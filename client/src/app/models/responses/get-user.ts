/**
 * This interface represents a specific user.
 */
export interface IGetUserResponse {
  /**
   * The unique identifier of the user.
   */
  id: string;

  /**
   * The name of the user.
   */
  name: string;

  /**
   * The email of the user.
   */
  email: string
}
