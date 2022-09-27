import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { CategoryRoutingModule } from './category-routing.module';
import { TreeTableModule } from 'primeng/treetable';

@NgModule({
  declarations: [],
  imports: [CommonModule, CategoryRoutingModule, TreeTableModule],
})
export class CategoryModule {}
