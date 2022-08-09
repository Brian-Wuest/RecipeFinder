import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { UserDataService } from './data/user-data.service';

@Injectable({
  providedIn: 'root',
})
export class UserService {
  userId: string;
  loggedIn: boolean = false;

  constructor(private router: Router, private userDataService: UserDataService) {
    this.userId = '';
  }

  /**
   * Determines if the user is currently logged into the server by verifying the HTTP only cookie.
   */
  determineIfUserIsLoggedIn() {
    this.userDataService.getMe().subscribe(response => {
      if (response && response.id) {
        this.userId = response.id;
        this.loggedIn = true;
      }
    });
  }

  /**
   * Logs the user out of the server by clearing the HTTP only cookie.
   * If the user is on a user-only screen then they are navigated back to root.
   */
  logOut() {
    this.userDataService.logout().subscribe(() => {
      this.loggedIn = false;

      if (this.router.url === '/user') {
        // The user logged out and was looking at user details, re-direct back to the root.
        this.router.navigate(['']);
      }
    });
  }
}
