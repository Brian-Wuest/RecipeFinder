import { Component, OnInit } from '@angular/core';
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

  constructor() {
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
    this.loggedIn = false;

    // TODO: Make sure to route back to home (recipe main page) if currently on the user page.
  }
}
