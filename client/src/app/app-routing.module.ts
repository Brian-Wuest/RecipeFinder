import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

const routes: Routes = [
  {
    path: 'category',
    loadChildren: () =>
      import('./modules/category/category.module').then(
        (m) => m.CategoryModule
      ),
  },
  {
    path: 'recipe',
    loadChildren: () =>
      import('./modules/recipe/recipe.module').then((m) => m.RecipeModule),
  },
  {
    path: 'user',
    loadChildren: () =>
      import('./modules/user-edit/user-edit.module').then(
        (m) => m.UserEditModule
      ),
  },
  {
    path: '',
    loadChildren: () =>
      import('./modules/recipe/recipe.module').then((m) => m.RecipeModule),
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
