import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable, Subject } from 'rxjs';
import { IGetCategoryResponse } from '../models/responses/get-category';
import { CategoryDataService } from './data/category-data.service';

@Injectable({
  providedIn: 'root',
})
export class CategoryService {
  private loadedCategories: BehaviorSubject<Array<IGetCategoryResponse>>;

  get categories() {
    return this.loadedCategories.asObservable();
  }

  constructor(private categoryDataService: CategoryDataService) {
    this.loadedCategories = new BehaviorSubject<Array<IGetCategoryResponse>>(undefined);
  }

  getCurrentCategories(): Array<IGetCategoryResponse> {
    return this.loadedCategories.value;
  }

  loadCategories(): void {
    this.categoryDataService.getCategories().subscribe(result => {
      // Assign the result to the property holding onto this state.
      this.loadedCategories.next(result);
    });
  }
}
