import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RouterModule, Routes } from '@angular/router';
import { ButtonModule } from 'primeng/button';
import { DialogModule } from 'primeng/dialog';
import { InputTextModule } from 'primeng/inputtext';
import { TableModule } from 'primeng/table';
import { ToolbarModule } from 'primeng/toolbar';
import { TreeSelectModule } from 'primeng/treeselect';
import { CategorySearchComponent } from './components/category-search/category-search.component';

const routes: Routes = [
  {
    path: '',
    component: CategorySearchComponent,
  },
];

@NgModule({
  declarations: [CategorySearchComponent],
  imports: [
    CommonModule,
    FormsModule,
    RouterModule.forChild(routes),
    TreeSelectModule,
    TableModule,
    InputTextModule,
    ButtonModule,
    ToolbarModule,
    DialogModule,
  ],
})
export class CategoryRoutingModule {}
