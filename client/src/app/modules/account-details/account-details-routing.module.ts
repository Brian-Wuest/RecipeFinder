/*!
//<copyright file="user-routing.module.ts" company="Symplr"
// Copyright 2022 Symplr. All rights reserved. Confidential and Proprietary.
//</copyright>
 */
import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { RouterModule, Routes } from '@angular/router';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { MenuModule } from 'primeng/menu';
import { TagModule } from 'primeng/tag';
import { AccountDetailsComponent } from './components/account-details/account-details.component';
import { UserEditComponent } from './components/user-edit/user-edit.component';

const routes: Routes = [
  {
    path: '',
    component: AccountDetailsComponent,
    children: [
      {
        path: 'recipe',
        loadChildren: () => import('../recipe/recipe.module').then(m => m.RecipeModule),
      },
      {
        path: 'edit',
        component: UserEditComponent,
      },
    ],
  },
];

@NgModule({
  declarations: [AccountDetailsComponent, UserEditComponent],
  imports: [CommonModule, RouterModule.forChild(routes), MenuModule, ReactiveFormsModule, FormsModule, InputTextModule, TagModule, ButtonModule],
})
export class AccountDetailsRoutingModule {}
