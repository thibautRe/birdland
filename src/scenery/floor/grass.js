import FloorTile from './floortile'
import getRandomColor from '../../utils/getRandomColor'

export default class GrassTile extends FloorTile {
  color = getRandomColor([65, 75], [170, 190], [80, 90])

  constructor(props) {
    super(props)
    this.fill()
  }
}
