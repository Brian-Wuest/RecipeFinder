import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { MenuItem } from 'primeng/api';
import { ILoginRequest } from 'src/app/models/requests/login-request';
import { UserDataService } from 'src/app/services/data/user-data.service';

@Component({
  selector: 'app-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.scss'],
})
export class NavigationComponent implements OnInit {
  items: Array<MenuItem>;
  userItems: Array<MenuItem>;
  loggedIn: boolean = false;

  constructor(private router: Router, private userDataService: UserDataService) {
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

  login() {
    const request = {
      name: "some_one",
      password: "password1"
    } as ILoginRequest;

    this.userDataService.login(request).subscribe(() => {
      this.loggedIn = true;
    });
  }

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
