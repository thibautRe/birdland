import FloorTile from './floortile'
import getRandomColor from '../../utils/getRandomColor'

export default class SeaTile extends FloorTile {
  color = getRandomColor([65, 75], [200, 210], [220, 240])

  constructor(props) {
    super(props)
    this.fill()
  }
}
