import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable, Subject } from 'rxjs';
import { INewCategory } from '../models/requests/new-category';
import { IUpdateCategory } from '../models/requests/update-category';
import { IGetCategory } from '../models/responses/get-category';
import { CategoryDataService } from './data/category-data.service';

@Injectable({
  providedIn: 'root',
})
export class CategoryService {
  private loadedCategories: BehaviorSubject<Array<IGetCategory>>;

  get categories() {
    return this.loadedCategories.asObservable();
  }

  constructor(private categoryDataService: CategoryDataService) {
    this.loadedCategories = new BehaviorSubject<Array<IGetCategory>>(undefined);
  }

  getCurrentCategories(): Array<IGetCategory> {
    return this.loadedCategories.value;
  }

  loadCategories(): void {
    this.categoryDataService.getCategories().subscribe(result => {
      // Assign the result to the property holding onto this state.
      this.loadedCategories.next(result);
    });
  }

  addCategory(category: INewCategory): void {
    // TODO: Add message service to get error messages from the server and show them to the user.
    this.categoryDataService.insertCategory(category).subscribe(result => {
      // Successfully added record.
      let currentCategories = [...this.getCurrentCategories()];
      currentCategories.push(result);

      this.loadedCategories.next(currentCategories);
    });
  }

  updateCategory(id: number, category: IUpdateCategory): void {
    // TODO: Add message service to get error messages from the server and show them to the user.
    this.categoryDataService.updateCategory(id, category).subscribe(result => {
      // Successfully updated record.
      let currentCategories = [...this.getCurrentCategories()];
      let index = currentCategories.findIndex(x => x.id === id);

      if (index !== -1) {
        let currentCategory = currentCategories[index];
        currentCategory.name = result.name;
        currentCategory.parentCategory = result.parentCategory;
      }

      this.loadedCategories.next(currentCategories);
    })
  }

  deleteCategory(id: number): void {
    // TODO: Add message service to get error messages from the server and show them to the user.
    this.categoryDataService.deleteCategory(id).subscribe(() => {
      // Successfully deleted record.
      let currentCategories = [...this.getCurrentCategories()];

      let index = currentCategories.findIndex(x => x.id === id);

      if (index !== -1) {
        currentCategories.splice(index, 1);
      }

      this.loadedCategories.next(currentCategories);
    });
  }
}
