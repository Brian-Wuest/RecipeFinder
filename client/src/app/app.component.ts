import { Component, OnDestroy, OnInit } from '@angular/core';
import { Subscription } from 'rxjs';
import { IGetCategory } from './models/responses/get-category';
import { CategoryService } from './services/category.service';
import { UserService } from './services/user.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit, OnDestroy {
  title = 'RecipeFinder';

  categoryLoader: Subscription;
  loading = true;

  /**
   *
   */
  constructor(private userService: UserService, private categoryService: CategoryService) {}

  ngOnInit(): void {
    this.categoryLoader = this.categoryService.categories.subscribe(categories => {
      this.categoriesUpdated(categories);
    });

    this.userService.determineIfUserIsLoggedIn();
    this.categoryService.loadCategories();
  }

  // istanbul ignore next
  ngOnDestroy(): void {
    if (this.categoryLoader) {
      this.categoryLoader.unsubscribe();
    }
  }

  categoriesUpdated(categories: Array<IGetCategory>): void {
    if (categories && categories.length > 0) {
      // We have categories, finished loading.
      this.loading = false;

      // Remove the subscription since we don't need it anymore.
      this.categoryLoader.unsubscribe();
      this.categoryLoader = null;
    }
  }
}
