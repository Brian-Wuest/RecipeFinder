import { AuthorizationRoleClassification } from '../enums/auth-role-classification.enum';

/**
 * Contains the authorization response for the current user.
 */
export interface IGetAuthorization {
  /**
   * The id of the application role.
   */
  id: number,

  /**
   * The name of the role.
   */
  name: string,

  /**
   * The classification for this role.
   */
  classification: AuthorizationRoleClassification
}
