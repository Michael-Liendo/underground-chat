type Direction = 'up' | 'down';

export default function ScrollTo(direction: Direction, element: HTMLElement) {
  element.scroll({
    behavior: 'smooth',
    top: direction === 'up' ? 0 : element.scrollHeight,
  });
}
