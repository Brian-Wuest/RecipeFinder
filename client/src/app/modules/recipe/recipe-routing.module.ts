import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { RecipeDetailsComponent } from './components/recipe-details/recipe-details.component';
import { RecipeSearchComponent } from './components/recipe-search/recipe-search.component';

const routes: Routes = [
  {
    path: '',
    component: RecipeSearchComponent,
  },
  {
    path: 'detail',
    component: RecipeDetailsComponent,
  },
];

@NgModule({
  declarations: [RecipeSearchComponent, RecipeDetailsComponent],
  imports: [CommonModule, RouterModule.forChild(routes)],
})
export class RecipeRoutingModule {}
