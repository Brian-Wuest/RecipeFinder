import { Component, OnDestroy, OnInit } from '@angular/core';
import { TreeNode } from 'primeng/api';
import { Subscription } from 'rxjs';
import { IGetCategory } from 'src/app/models/responses/get-category';
import { CategoryService } from 'src/app/services/category.service';

@Component({
  selector: 'app-category-search',
  templateUrl: './category-search.component.html',
  styleUrls: ['./category-search.component.scss'],
})
export class CategorySearchComponent implements OnInit, OnDestroy {
  subscriptions: Array<Subscription>;
  recipeCategories: Array<TreeNode>;

  constructor(private categoryService: CategoryService) {
    this.subscriptions = new Array<Subscription>();
    this.recipeCategories = new Array<TreeNode>();
  }

  ngOnInit(): void {
    this.setupSubscriptions();
  }

  // istanbul ignore next
  ngOnDestroy(): void {
    for (let subscription of this.subscriptions) {
      subscription.unsubscribe();
    }

    this.subscriptions = new Array<Subscription>();
  }

  setupSubscriptions(): void {
    this.subscriptions.push(
      this.categoryService.categories.subscribe(categories => {
        this.reBuildCategoryTree(categories);
      })
    );
  }

  reBuildCategoryTree(categories: Array<IGetCategory>) {
    // Only do this when the array is truthy;
    if (categories) {
      const categoryTree = new Array<TreeNode>();

      for (let category of categories) {
        let treeNode = {
          key: category.id.toString(),
          label: category.name,
          data: category,
        } as TreeNode;

        if (category.parentCategory) {
          // This is a child category, don't add it to the main list, add it to the parent node instead.
          for (let parentCategoryNode of categoryTree) {
            if (parentCategoryNode.data.id === category.parentCategory.id) {
              // Found the parent category, add the current node to the parent's children array.
              if (!parentCategoryNode.children) {
                parentCategoryNode.children = new Array<TreeNode>();
              }

              parentCategoryNode.droppable = true;
              parentCategoryNode.children.push(treeNode);
              break;
            }
          }
        } else {
          treeNode.droppable = true;
          categoryTree.push(treeNode);
        }
      }

      this.recipeCategories = [...categoryTree];
    }
  }
}
