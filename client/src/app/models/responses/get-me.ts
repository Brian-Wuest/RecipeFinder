/**
 * The response for getting information about the current user.
 */
export interface IGetMeResponse {
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
  email: string;
}
