/*!
//<copyright file="category-row.ts" company="Symplr"
// Copyright 2022 Symplr. All rights reserved. Confidential and Proprietary.
//</copyright>
 */

import { TreeNode } from 'primeng/api';
import { IGetCategory } from 'src/app/models/responses/get-category';

export interface ICategoryRow extends IGetCategory {

  /**
   * The tree node representation of this category's parent.
   */
  parentTreeNode: TreeNode
}
