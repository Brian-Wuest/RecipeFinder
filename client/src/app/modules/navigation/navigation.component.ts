import { Component, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { MenuItem } from 'primeng/api';
import { OverlayPanel } from 'primeng/overlaypanel';
import { Subscription } from 'rxjs';
import { ILoginRequest } from 'src/app/models/requests/login-request';
import { UserDataService } from 'src/app/services/data/user-data.service';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.scss'],
})
export class NavigationComponent implements OnInit, OnDestroy {
  items: Array<MenuItem>;
  userItems: Array<MenuItem>;
  loginFormGroup: FormGroup;
  loginNameKey = 'loginName';
  passwordKey = 'password';
  userName: string;
  private userNameChangedSubscription: Subscription

  @ViewChild('op')
  loginPanel: OverlayPanel | undefined;

  constructor(private userDataService: UserDataService, protected userService: UserService) {
    this.loginFormGroup = new FormGroup({
      loginName: new FormControl(''),
      password: new FormControl(''),
    });

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
          this.userService.logOut();
        },
      },
    ];
  }

  ngOnInit(): void {
    this.userNameChangedSubscription = this.userService.userNameUpdated.subscribe(value => {
      this.userName = value;
    });
  }

  ngOnDestroy(): void {
    if (this.userNameChangedSubscription) {
      this.userNameChangedSubscription.unsubscribe();
    }
  }

  login() {
    const request = {
      name: this.loginFormGroup.get(this.loginNameKey)?.value,
      password: this.loginFormGroup.get(this.passwordKey)?.value,
    } as ILoginRequest;

    // Make sure to clear out the controls ASAP.
    this.loginFormGroup.get(this.loginNameKey)?.setValue('');
    this.loginFormGroup.get(this.passwordKey)?.setValue('');

    // TODO: Show error messages should the login fail.
    this.userService.login(request);

    this.loginPanel?.hide();
  }
}
