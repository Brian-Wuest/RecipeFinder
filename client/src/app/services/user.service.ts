import { HttpErrorResponse } from '@angular/common/http';
import { EventEmitter, Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { AuthorizationRoleClassification } from '../models/enums/auth-role-classification.enum';
import { ILoginRequest } from '../models/requests/login-request';
import { IGetAuthorization } from '../models/responses/get-authorization';
import { IGetMeResponse } from '../models/responses/get-me';
import { AuthorizationDataService } from './data/authorization-data.service';
import { UserDataService } from './data/user-data.service';

@Injectable({
  providedIn: 'root',
})
export class UserService {
  currentUser: IGetMeResponse;
  authorizationRole: IGetAuthorization;
  loggedIn: boolean = false;
  userNameUpdated: EventEmitter<string>;

  constructor(private router: Router, private userDataService: UserDataService, private authDataService: AuthorizationDataService) {
    this.currentUser = null;
    this.userNameUpdated = new EventEmitter<string>();
  }

  /**
   * Determines if the user is currently logged into the server by verifying the HTTP only cookie.
   */
  determineIfUserIsLoggedIn(): void {
    this.userDataService.getMe().subscribe(response => {
      if (response && response.id) {
        this.currentUser = response;
        this.userNameUpdated.emit(this.currentUser.name);

        const eventSubscription = this.getAuthorization().subscribe(() => {
          this.loggedIn = true;
          eventSubscription.unsubscribe();
        });
      }
    }, (err:HttpErrorResponse)=>{
      // Current user not logged in.
      // Don't do anything with this error.
    });
  }

  login(request: ILoginRequest): void {
    this.userDataService.login(request).subscribe(result => {
      this.currentUser = result;
      this.userNameUpdated.emit(this.currentUser.name);

      const eventSubscription = this.getAuthorization().subscribe(() => {
        this.loggedIn = true;
        eventSubscription.unsubscribe();
      });
    });
  }

  /**
   * Logs the user out of the server by clearing the HTTP only cookie.
   * If the user is on a user-only screen then they are navigated back to root.
   */
  logOut() {
    this.userDataService.logout().subscribe(() => {
      this.loggedIn = false;
      this.userNameUpdated.emit('');

      if (this.router.url === '/user') {
        // The user logged out and was looking at user details, re-direct back to the root.
        this.router.navigate(['']);
      }
    });
  }

  getAuthorization(): EventEmitter<boolean> {
    const eventEmitter = new EventEmitter<boolean>();

    this.authDataService.getAuthorizationRole().subscribe(authResponse => {
      this.authorizationRole = authResponse;
      eventEmitter.next(true);
    });

    return eventEmitter;
  }

  /**
   * Determines if the specified role is lower or matches the user's current role.
   * @param role The role to check against.
   * @returns True if the current user's role matches or is higher than the desired role.
   */
  hasNecessaryRole(role: AuthorizationRoleClassification): boolean {
    return this.authorizationRole && this.authorizationRole.classification >= role;
  }
}
