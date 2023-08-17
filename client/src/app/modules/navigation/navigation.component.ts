import { Component, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { Router } from '@angular/router';
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
  isLightTheme: boolean;
  private userNameChangedSubscription: Subscription;

  @ViewChild('op')
  loginPanel: OverlayPanel | undefined;

  constructor(private userDataService: UserDataService, protected userService: UserService, private router: Router) {
    this.isLightTheme = true;

    this.loginFormGroup = new FormGroup({
      loginName: new FormControl(''),
      password: new FormControl(''),
    });

    this.items = [
      {
        label: 'Home',
        icon: "pi pi-home",
        routerLink: 'recipe',
      },
      {
        label: 'Categories',
        icon: "pi pi-tags",
        routerLink: 'category',
      },
    ];

    this.userItems = [
      {
        label: 'Account Details',
        routerLink: 'user',
        icon: "pi pi-id-card"
      },
      {
        label: 'Log Out',
        icon: "pi pi-sign-out",
        command: () => {
          this.userService.logOut();
          this.router.navigate(['']);
        },
      },
    ];
  }

  ngOnInit(): void {
    this.userNameChangedSubscription = this.userService.userNameUpdated.subscribe(value => {
      this.userName = value;
    });

    var isDark = localStorage.getItem("isDark")?.toLowerCase() === 'true';

    if (isDark) {
      this.isLightTheme = false;
      this.darkToggleChanged();
    }
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

  darkToggleChanged() {
    if (this.isLightTheme) {
      this.changeLightDarkTheme('theme-css', 'light-theme');
    }
    else {
      this.changeLightDarkTheme('theme-css', 'dark-theme');
    }
  }

  changeLightDarkTheme(id: string, value: string) {
    this.changeLightDarkElement(id, value);
    localStorage.setItem("isDark", (!this.isLightTheme).valueOf().toString());
  }

  changeLightDarkLayout(id: string, value: string) {
    this.changeLightDarkElement(id, value);
  }

  changeLightDarkElement(id:string, value: string) {
    const element = document.getElementById(id);
    const urlTokens = element.getAttribute('href').split('/');
    urlTokens[urlTokens.length - 1] = value + '.css';
    const newURL = urlTokens.join('/');
    this.replaceLink(element, newURL);
  }

  replaceLink(linkElement: HTMLElement, href: string) {
    const id = linkElement.getAttribute('id');
    const cloneLinkElement = linkElement.cloneNode(true) as HTMLElement;
    cloneLinkElement.setAttribute('href', href);
    cloneLinkElement.setAttribute('id', id + '-clone');
    linkElement.parentNode.insertBefore(cloneLinkElement, linkElement.nextSibling);

    cloneLinkElement.addEventListener('load', () => {
      linkElement.remove();
      cloneLinkElement.setAttribute('id', id);
    });
  }
}
