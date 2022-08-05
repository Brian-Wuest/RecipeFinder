import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { CategoryDetailsComponent } from './components/category-details/category-details.component';
import { CategorySearchComponent } from './components/category-search/category-search.component';

const routes: Routes = [
  {
    path: '',
    component: CategorySearchComponent,
  },
  {
    path: 'detail',
    component: CategoryDetailsComponent,
  },
];

@NgModule({
  declarations: [CategorySearchComponent, CategoryDetailsComponent],
  imports: [CommonModule, RouterModule.forChild(routes)],
})
export class CategoryRoutingModule {}
