import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { MenuModule } from 'primeng/menu';
import { UserEditComponent } from './user-edit.component';

const routes: Routes = [
  {
    path: '',
    component: UserEditComponent,
  },
];

@NgModule({
  declarations: [UserEditComponent],
  imports: [CommonModule, RouterModule.forChild(routes), MenuModule],
})
export class UserRoutingModule {}
