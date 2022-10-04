import { Component, OnDestroy, OnInit } from '@angular/core';
import { TreeNode } from 'primeng/api';
import { Subscription } from 'rxjs';
import { INewCategory } from 'src/app/models/requests/new-category';
import { IUpdateCategory } from 'src/app/models/requests/update-category';
import { IGetCategory } from 'src/app/models/responses/get-category';
import { CategoryService } from 'src/app/services/category.service';
import { ICategoryRow } from '../../models/category-row';

@Component({
  selector: 'app-category-search',
  templateUrl: './category-search.component.html',
  styleUrls: ['./category-search.component.scss'],
})
export class CategorySearchComponent implements OnInit, OnDestroy {
  subscriptions: Array<Subscription>;
  recipeCategories: Array<TreeNode>;
  categoryRows: Array<ICategoryRow>;
  categoryDialogShown: boolean;
  newCategory: ICategoryRow;

  constructor(private categoryService: CategoryService) {
    this.subscriptions = new Array<Subscription>();
    this.recipeCategories = new Array<TreeNode>();
    this.categoryRows = new Array<ICategoryRow>();
    this.categoryDialogShown = false;
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
      const rows = new Array<ICategoryRow>();

      for (let category of categories) {
        let treeNode = {
          key: category.id.toString(),
          label: category.name,
          data: category,
        } as TreeNode;

        let parentNode: TreeNode = null;

        if (category.parentCategory) {
          // This is a child category, don't add it to the main list, add it to the parent node instead.
          for (let parentCategoryNode of categoryTree) {
            if (parentCategoryNode.data.id === category.parentCategory.id) {
              // Found the parent category, add the current node to the parent's children array.
              if (!parentCategoryNode.children) {
                parentCategoryNode.children = new Array<TreeNode>();
              }

              parentCategoryNode.children.push(treeNode);
              parentNode = parentCategoryNode;
              break;
            }
          }
        } else {
          categoryTree.push(treeNode);
        }

        const rowData = {
          id: category.id,
          name: category.name,
          parentCategory: category.parentCategory,
          parentTreeNode: parentNode,
        } as ICategoryRow;

        rows.push(rowData);
      }

      this.recipeCategories = [...categoryTree];
      this.categoryRows = [...rows];
    }
  }

  updateParentCategory(row: ICategoryRow, parentNode: TreeNode) {
    if (parentNode) {
      row.parentCategory = parentNode.data;
      row.parentTreeNode = parentNode;
    } else {
      row.parentCategory = null;
      row.parentTreeNode = null;
    }
  }

  saveUpdate(row: ICategoryRow) {
    if (row && row.name) {
      const request = {
        name: row.name,
        parentCategoryId: row.parentCategory?.id,
      } as IUpdateCategory;

      // This will trigger a re-build of the table when successful.
      this.categoryService.updateCategory(row.id, request);
    }
  }

  deleteCategory(row: ICategoryRow) {
    if (row && row.id) {
      // This will trigger a re-build of the table when successful.
      this.categoryService.deleteCategory(row.id);
    }
  }

  saveNewCategory() {
    if (this.newCategory && this.newCategory.name) {
      const request = {
        name: this.newCategory.name,
        parentCategoryId: this.newCategory.parentCategory?.id,
      } as INewCategory;

      // This will trigger a re-build of the table when successful.
      this.categoryService.addCategory(request);
      this.categoryDialogShown = false;
    }
  }

  showNewCategoryDialog() {
    this.newCategory = {} as ICategoryRow;
    this.categoryDialogShown = true;
  }

  hideDialog() {
    this.categoryDialogShown = false;
  }
}
