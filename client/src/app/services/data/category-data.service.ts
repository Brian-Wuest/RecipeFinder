import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { INewCategory } from 'src/app/models/requests/new-category';
import { IUpdateCategory } from 'src/app/models/requests/update-category';
import { IGetCategory } from 'src/app/models/responses/get-category';
import { RouteConfigService } from './route-config.service';

@Injectable({
  providedIn: 'root',
})
export class CategoryDataService {
  constructor(private routeConfig: RouteConfigService, private httpClient: HttpClient) {}

  getCategories(): Observable<Array<IGetCategory>> {
    return this.httpClient.get<Array<IGetCategory>>(this.routeConfig.baseCategoryPath, { withCredentials: true });
  }

  insertCategory(request: INewCategory): Observable<IGetCategory> {
    return this.httpClient.post<IGetCategory>(this.routeConfig.baseCategoryPath, request, { withCredentials: true });
  }

  updateCategory(id: number, request: IUpdateCategory): Observable<IGetCategory> {
    const route = this.routeConfig.baseCategoryPath + '/' + id.toString();

    return this.httpClient.put<IGetCategory>(route, request, { withCredentials: true });
  }

  deleteCategory(id: number): Observable<string> {
    const route = this.routeConfig.baseCategoryPath + '/' + id.toString();

    return this.httpClient.delete<string>(route, { withCredentials: true });
  }
}
