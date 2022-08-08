import { Component, OnInit, ViewChild } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { Router } from '@angular/router';
import { MenuItem } from 'primeng/api';
import { OverlayPanel } from 'primeng/overlaypanel';
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
  loginFormGroup: FormGroup;
  loginNameKey = 'loginName';
  passwordKey = 'password';

  @ViewChild('op')
  loginPanel: OverlayPanel | undefined

  constructor(private router: Router, private userDataService: UserDataService) {
    this.loginFormGroup = new FormGroup(
      {
        loginName: new FormControl(''),
        password: new FormControl('')
      },
    );

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
      name: this.loginFormGroup.get(this.loginNameKey)?.value,
      password: this.loginFormGroup.get(this.passwordKey)?.value,
    } as ILoginRequest;

    this.userDataService.login(request).subscribe(() => {
      this.loggedIn = true;

      this.loginPanel?.hide();
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
