/* tslint:disable */
/* eslint-disable */
export class Pager {
  free(): void;
/**
* @param {Uint8Array} before_image 
* @param {Uint8Array} after_image 
*/
  static initialize(before_image: Uint8Array, after_image: Uint8Array): void;
/**
* @param {number} interval 
* @returns {any} 
*/
  static up(interval: number): any;
/**
* @param {number} interval 
* @returns {any} 
*/
  static right(interval: number): any;
/**
* @param {number} interval 
* @returns {any} 
*/
  static down(interval: number): any;
/**
* @param {number} interval 
* @returns {any} 
*/
  static left(interval: number): any;
}
