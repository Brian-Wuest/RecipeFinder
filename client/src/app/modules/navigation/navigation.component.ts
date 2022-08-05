import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { MenuItem } from 'primeng/api';

@Component({
  selector: 'app-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.scss'],
})
export class NavigationComponent implements OnInit {
  items: Array<MenuItem>;
  userItems: Array<MenuItem>;
  loggedIn: boolean = false;

  constructor(private router: Router) {
    this.items = [
      {
        label: 'Home',
        routerLink: 'recipe',
      },
      {
        label: 'Categories',
        routerLink: 'category',
      },
    ];

    this.userItems = [
      {
        label: 'Account Details',
        routerLink: 'user',
      },
      {
        label: 'Log Out',
        command: () => {
          this.logOut();
        },
      },
    ];
  }

  ngOnInit(): void {}

  logOut() {
    // TODO: Send request to the server to log the user out to remove the login cookie from the browser.
    this.loggedIn = false;

    if (this.router.url === '/user') {
      // The user logged out and was looking at user details, re-direct back to the root.
      this.router.navigate(['']);
    }
  }
}
