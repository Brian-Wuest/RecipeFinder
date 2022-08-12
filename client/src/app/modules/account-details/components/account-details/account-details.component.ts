import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { MenuItem } from 'primeng/api';

@Component({
  selector: 'app-account-details',
  templateUrl: './account-details.component.html',
  styleUrls: ['./account-details.component.scss'],
})
export class AccountDetailsComponent implements OnInit {
  menuItems: Array<MenuItem>;

  constructor(private router: Router) {
    this.menuItems = [
      {
        label: 'My Profile',
        icon: 'pi pi-user',
        routerLink: 'edit',
      },
      {
        label: 'My Recipes',
        icon: 'pi pi-book',
        routerLink: 'recipe',
        queryParams: {
          currentUser: 'true',
        },
      },
    ];
  }

  ngOnInit(): void {
    // Automatically route to the user edit screen.
    this.router.navigate(['user', 'edit']);
  }
}
