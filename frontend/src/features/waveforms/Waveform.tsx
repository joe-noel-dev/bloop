import React, {useEffect, useRef, useState} from 'react';
import styled from 'styled-components';
import {requestWaveformRequest} from '../../api/request';
import {WaveformData, WaveformPeaks} from '../../model/waveform';
import {useCore} from '../core/use-core';
import {useSampleWithId} from '../samples/sample-hooks';
import {useWaveformData} from './waveform-hooks';

const Container = styled.div`
  background: ${(props) => props.theme.colours.cardLayer};
  width: 100%;
  height: 100%;
`;

interface Props {
  sampleId?: string;
  start?: number;
  end?: number;
}

function choosePeaks(
  waveformData: WaveformData,
  width: number,
  algorithm: string,
  channel: number,
  start: number,
  end: number
): WaveformPeaks | undefined {
  const peaks = waveformData.peaks.filter(
    (peaks) =>
      peaks.properties.algorithm === algorithm &&
      peaks.properties.channel === channel
  );

  if (peaks.length === 0) return undefined;

  let bestPeaks = peaks[0];

  peaks.forEach((peak) => {
    if (
      peak.values.length > width / (end - start) &&
      peak.values.length < bestPeaks.values.length
    )
      bestPeaks = peak;
  });

  return bestPeaks;
}

function buildMinMaxPath(
  min: WaveformPeaks,
  max: WaveformPeaks,
  width: number,
  height: number,
  start: number,
  end: number
): Path2D {
  const minPath = new Path2D();
  const maxPath = new Path2D();

  minPath.moveTo(0, 0);
  maxPath.moveTo(0, 0);

  for (
    let index = 0;
    index < min.values.length && index < max.values.length;
    ++index
  ) {
    const x = index / min.values.length;
    minPath.lineTo(x, -min.values[index]);
    maxPath.lineTo(x, -max.values[index]);
  }

  minPath.lineTo(1, 0);
  maxPath.lineTo(1, 0);

  minPath.lineTo(0, 0);
  maxPath.lineTo(0, 0);

  const xScale = width / (end - start);
  const yScale = 0.5 * height;

  const xShift = -start * xScale;
  const yShift = yScale;

  let transform = new DOMMatrix([xScale, 0, 0, yScale, xShift, yShift]);

  const path = new Path2D();
  path.addPath(minPath, transform);
  path.addPath(maxPath, transform);

  return path;
}

function pathFromWaveformData(
  waveformData: WaveformData,
  width: number,
  height: number,
  start: number,
  end: number
): Path2D {
  const max = choosePeaks(waveformData, width, 'max', 0, start, end);
  const min = choosePeaks(waveformData, width, 'min', 0, start, end);

  if (!min || !max || min.values.length !== max.values.length)
    return new Path2D();

  return buildMinMaxPath(min, max, width, height, start, end);
}

export const Waveform = (props: Props) => {
  const core = useCore();
  const waveformData = useWaveformData(props.sampleId || '');
  const container = useRef<HTMLDivElement>(null);
  const canvas = useRef<HTMLCanvasElement>(null);
  const [width, setWidth] = useState<number>(0);
  const [height, setHeight] = useState<number>(0);
  const sample = useSampleWithId(props.sampleId || '');

  useEffect(() => {
    if (props.sampleId && !waveformData)
      core?.sendRequest(requestWaveformRequest(props.sampleId));
  }, [props.sampleId, waveformData, core]);

  useEffect(() => {
    if (!canvas.current) return;
    const context = canvas.current.getContext('2d');
    if (!context) return;

    context.clearRect(0, 0, width, height);

    if (waveformData) {
      const path = pathFromWaveformData(
        waveformData,
        width,
        height,
        props.start || 0,
        props.end || 1
      );
      context.fillStyle = '#ffab91';
      context.fill(path);
    }
  }, [
    waveformData,
    props.sampleId,
    width,
    height,
    props.start,
    props.end,
    sample,
  ]);

  useEffect(() => {
    setWidth(container.current?.clientWidth || 0);
    setHeight(container.current?.clientHeight || 0);
  }, [setWidth]);

  return (
    <Container ref={container}>
      <WaveformCanvas ref={canvas} width={width} height={height} />
    </Container>
  );
};

const WaveformCanvas = styled.canvas`
  width: 100%;
  height: 100%;
`;
