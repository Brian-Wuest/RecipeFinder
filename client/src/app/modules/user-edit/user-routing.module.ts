import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { UserEditComponent } from './user-edit.component';

const routes: Routes = [
  {
    path: '',
    component: UserEditComponent,
  },
];

@NgModule({
  declarations: [UserEditComponent],
  imports: [CommonModule, RouterModule.forChild(routes)],
})
export class UserRoutingModule {}
