import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { INewCategory } from 'src/app/models/requests/new-category';
import { IUpdateCategory } from 'src/app/models/requests/update-category';
import { IGetCategoryResponse } from 'src/app/models/responses/get-category';
import { RouteConfigService } from './route-config.service';

@Injectable({
  providedIn: 'root',
})
export class CategoryDataService {
  constructor(private routeConfig: RouteConfigService, private httpClient: HttpClient) {}

  getCategories(): Observable<Array<IGetCategoryResponse>> {
    return this.httpClient.get<Array<IGetCategoryResponse>>(this.routeConfig.baseCategoryPath, { withCredentials: true });
  }

  insertCategory(request: INewCategory): Observable<IGetCategoryResponse> {
    return this.httpClient.post<IGetCategoryResponse>(this.routeConfig.baseCategoryPath, request, { withCredentials: true });
  }

  updateCategory(id: number, request: IUpdateCategory): Observable<IGetCategoryResponse> {
    const route = this.routeConfig.baseCategoryPath + '/' + id.toString();

    return this.httpClient.put<IGetCategoryResponse>(route, request, { withCredentials: true });
  }

  deleteCategory(id: number): Observable<string> {
    const route = this.routeConfig.baseCategoryPath + '/' + id.toString();

    return this.httpClient.delete<string>(route, { withCredentials: false });
  }
}
