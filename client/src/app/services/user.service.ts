import { EventEmitter, Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { ILoginRequest } from '../models/requests/login-request';
import { IGetMeResponse } from '../models/responses/get-me';
import { UserDataService } from './data/user-data.service';

@Injectable({
  providedIn: 'root',
})
export class UserService {
  currentUser: IGetMeResponse;
  loggedIn: boolean = false;
  userNameUpdated: EventEmitter<string>;

  constructor(private router: Router, private userDataService: UserDataService) {
    this.currentUser = null;
    this.userNameUpdated = new EventEmitter<string>();
  }

  /**
   * Determines if the user is currently logged into the server by verifying the HTTP only cookie.
   */
  determineIfUserIsLoggedIn() {
    this.userDataService.getMe().subscribe(response => {
      if (response && response.id) {
        this.currentUser = response;
        this.userNameUpdated.emit(this.currentUser.name);
        this.loggedIn = true;
      }
    });
  }

  login(request: ILoginRequest) {
    this.userDataService.login(request).subscribe(result => {
      this.currentUser = result;
      this.userNameUpdated.emit(this.currentUser.name);
      this.loggedIn = true;
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
}
